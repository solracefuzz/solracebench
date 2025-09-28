use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct DexAccount {
    token_a: u64,
    token_b: u64,
    fee_rate: u8,
    price_history: [f64; 2],
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
        let init_data = DexAccount {
            token_a: 0,
            token_b: 0,
            fee_rate: 3, // 0.3%
            price_history: [0.0; 2],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = DexAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 添加流动性
            let a = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            let b = u64::from_le_bytes(instruction_data[9..17].try_into().unwrap());
            data.token_a += a;
            data.token_b += b;
            data.price_history[0] = data.token_a as f64 / data.token_b as f64;
        },
        1 => { // 执行交易
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            let fee = amount * data.fee_rate as u64 / 1000;
            let actual_amount = amount - fee;
            
            data.token_a = data.token_a.checked_sub(actual_amount).unwrap();
            data.token_b = data.token_b.checked_add(
                (actual_amount as f64 * data.price_history[0]) as u64
            ).unwrap();
            
            data.price_history[1] = data.price_history[0];
            data.price_history[0] = data.token_a as f64 / data.token_b as f64;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}