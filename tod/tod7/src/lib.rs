use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MathAccount {
    value: i64,
    history: [String; 2],
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // 验证账户所有权
    if account.owner != program_id {
        msg!("Invalid account owner");
        return Err(ProgramError::IncorrectProgramId);
    }

    // 初始化空账户
    if account.data_len() == 0 {
        let init_data = MathAccount {
            value: 100,
            history: ["init".to_string(), "init".to_string()],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        msg!("Account initialized");
        return Ok(());
    }

    // 解析指令
    let instruction = instruction_data[0];
    let mut data = MathAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 平方运算
            data.value = data.value.pow(2);
            data.history[0] = "square".to_string();
        },
        1 => { // 取反运算
            data.value = -data.value;
            data.history[1] = "negate".to_string();
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}