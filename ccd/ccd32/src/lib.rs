use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
    msg,
};

#[repr(C)]
struct Staking {
    start_timestamp: i64,
    last_epoch_start: i64,
    total_rewards: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let stake_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(clock_account)?;

    // 初始化质押
    if stake_account.data_is_empty() {
        let staking = Staking {
            start_timestamp: clock.epoch_start_timestamp,
            last_epoch_start: clock.epoch_start_timestamp,
            total_rewards: 0,
        };
        let datalen = std::mem::size_of::<Staking>();
        stake_account.realloc(datalen, false);
        let mut data = stake_account.data.borrow_mut();

        unsafe { std::ptr::write(data.as_mut_ptr() as *mut Staking, staking) };
        return Ok(());
    }

    // 计算奖励
    let mut data = stake_account.data.borrow_mut();
    let mut staking = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const Staking) };

    let time_diff = clock.epoch_start_timestamp - staking.last_epoch_start;

    if time_diff > 1 {
        staking.total_rewards += (time_diff as u64) * 10;
    }

    unsafe { std::ptr::write_unaligned(data.as_mut_ptr() as *mut Staking, staking) };

    Ok(())
}