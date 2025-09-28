use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct PricingAccount {
    base_price: u64,
    dynamic_factor: f64,
    transaction_count: u32,
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

    if account.data_len() == 0 {
        let init_data = PricingAccount {
            base_price: 100,
            dynamic_factor: 1.0,
            transaction_count: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = PricingAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 需求调整
            let adjustment = f64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            data.dynamic_factor *= 1.0 + adjustment;
            data.transaction_count += 1;
        },
        1 => { // 基础价格更新
            let new_price = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            data.base_price = new_price;
            data.transaction_count += 1;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}