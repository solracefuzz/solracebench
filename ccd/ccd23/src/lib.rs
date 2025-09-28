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
struct StakingPool {
    start_time: i64,
    last_update: i64,
    total_staked: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    let pool_account = next_account_info(accounts_iter)?;
    let staker_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(clock_account)?;

    // 初始化质押池
    if pool_account.data_is_empty() {
        let pool = StakingPool {
            start_time: clock.unix_timestamp,
            last_update: clock.unix_timestamp,
            total_staked: 0,
        };
        let datalen = std::mem::size_of::<StakingPool>();
        pool_account.realloc(datalen, false);
        let mut data = pool_account.data.borrow_mut();
        unsafe { std::ptr::write(data.as_mut_ptr() as *mut StakingPool, pool) };
        return Ok(());
    }

    // 处理质押操作
    let amount = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    
    let mut data = pool_account.data.borrow_mut();
    let mut pool = unsafe {
        std::ptr::read_unaligned(data.as_ptr() as *const StakingPool)
    };

    let elapsed_time = clock.unix_timestamp - pool.last_update;
    let rewards = pool.total_staked * elapsed_time as u64 / 86400;
    
    // 模拟转账操作
    if rewards > 1 {
        msg!("Distributing {} rewards", rewards);
    }

    pool.total_staked += amount;
    
    unsafe {
        std::ptr::write_unaligned(
            data.as_mut_ptr() as *mut StakingPool,
            pool
        );
    }

    Ok(())
}