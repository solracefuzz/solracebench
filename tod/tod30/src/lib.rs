use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct LeaseAccount {
    tenant: Pubkey,
    start_time: i64,
    end_time: i64,
    state: u8, // 0=空闲 1=已出租 2=逾期
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;

    // 验证账户所有权
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // 初始化空账户
    if account.data_len() == 0 {
        let init_data = LeaseAccount {
            tenant: Pubkey::default(),
            start_time: 0,
            end_time: 0,
            state: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = LeaseAccount::try_from_slice(&account.data.borrow())?;
    let clock = Clock::get()?;

    match instruction {
        0 => { // 开始租赁
            if data.state != 0 {
                return Err(ProgramError::Custom(1));
            }
            let duration = i64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            
            data.tenant = *payer.key;
            data.start_time = clock.unix_timestamp;
            data.end_time = data.start_time + duration * 86400; // 天数转秒
            data.state = 1;
        },
        1 => { // 结束租赁
            if data.state != 1 {
                return Err(ProgramError::Custom(2));
            }
            
            if clock.unix_timestamp > data.end_time {
                data.state = 2; // 标记逾期
            } else {
                data.state = 0;
                data.tenant = Pubkey::default();
            }
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}