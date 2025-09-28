use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Marketplace {
    price: u64,
    discount_threshold: u64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum MarketInstruction {
    ApplyDiscount,
    SetPrice(u64),
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // 初始化检测
    if account.data.borrow().is_empty() {
        let init_account = Marketplace {
            price: 0,
            discount_threshold: 100,
        };
        let datalen = borsh::object_length(&init_account).unwrap();
        account.realloc(datalen, false);
        init_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let mut market = Marketplace::try_from_slice(&account.data.borrow())?;
    let instruction = MarketInstruction::try_from_slice(instruction_data)?;

    match instruction {
        MarketInstruction::ApplyDiscount => {
            if market.price > market.discount_threshold {
                market.price = market.price.checked_mul(75).unwrap() / 100;
            }
        }
        MarketInstruction::SetPrice(new_price) => {
            market.price = new_price;
            market.discount_threshold = new_price.checked_mul(2).unwrap();
        }
    }

    market.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}