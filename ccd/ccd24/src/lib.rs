use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

#[repr(C)]
struct TimeLock {
    unlock_slot: u64,
    locked_amount: u64,
    owner: [u8; 32],
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    let lock_account = next_account_info(accounts_iter)?;
    let owner_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    // 初始化时间锁
    if lock_account.data_is_empty() {
        let unlock_slot = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
        
        let time_lock = TimeLock {
            unlock_slot,
            locked_amount: 0,
            owner: owner_account.key.to_bytes(),
        };
        let datalen = std::mem::size_of::<TimeLock>();
        lock_account.realloc(datalen, false);
        let mut data = lock_account.data.borrow_mut();

        unsafe { std::ptr::write(data.as_mut_ptr() as *mut TimeLock, time_lock) };
        return Ok(());
    }

    // 处理提款操作
    let clock = Clock::get()?;
    
    let mut data = lock_account.data.borrow_mut();
    let mut lock = unsafe {
        std::ptr::read_unaligned(data.as_ptr() as *const TimeLock)
    };

    if clock.slot < lock.unlock_slot {
        return Err(ProgramError::InvalidArgument);
    }

    // 模拟转账操作
    lock.locked_amount = 0;
    
    unsafe {
        std::ptr::write_unaligned(
            data.as_mut_ptr() as *mut TimeLock,
            lock
        );
    }

    Ok(())
}