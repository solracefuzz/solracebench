use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct DiscountAccount {
    purchase_count: u32,
    discount_rate: f32,
    last_purchase: u64,
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
        let init_data = DiscountAccount {
            purchase_count: 0,
            discount_rate: 0.0,
            last_purchase: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = DiscountAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 应用折扣
            let original_price = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            data.last_purchase = (original_price as f32 * (1.0 - data.discount_rate)) as u64;
        },
        1 => { // 更新折扣率
            data.purchase_count += 1;
            data.discount_rate = if data.purchase_count > 10 {
                0.2
            } else if data.purchase_count > 5 {
                0.1
            } else {
                0.0
            };
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}