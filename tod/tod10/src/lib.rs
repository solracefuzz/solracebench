use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct TimeSeriesAccount {
    start_time: i64,
    end_time: i64,
    duration: i64,
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
        let init_data = TimeSeriesAccount {
            start_time: 0,
            end_time: 0,
            duration: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        msg!("Account initialized");
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = TimeSeriesAccount::try_from_slice(&account.data.borrow())?;
    let clock = Clock::get()?; // 获取系统时间

    match instruction {
        0 => { // 记录开始时间
            data.start_time = clock.unix_timestamp;
            msg!("Start time recorded");
        },
        1 => { // 记录结束时间并计算持续时间
            data.end_time = clock.unix_timestamp;
            data.duration = data.end_time - data.start_time;
            msg!("Duration calculated");
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}