use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

#[repr(C)]
struct SlotLock {
    unlock_slot: u64,
    locked_amount: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let lock_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    // 初始化锁定
    if lock_account.data_is_empty() {
        let duration_slots = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
        let clock = Clock::from_account_info(clock_account)?;
        
        let lock = SlotLock {
            unlock_slot: clock.slot + duration_slots,
            locked_amount: 0,
        };
        let datalen = std::mem::size_of::<SlotLock>();
        lock_account.realloc(datalen, false);
        let mut data = lock_account.data.borrow_mut();
        unsafe { std::ptr::write(data.as_mut_ptr() as *mut SlotLock, lock) };
        return Ok(());
    }

    // 提款操作
    let clock = Clock::from_account_info(clock_account)?;
    let mut data = lock_account.data.borrow_mut();
    let mut lock = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const SlotLock) };

    if clock.slot < lock.unlock_slot {
        return Err(ProgramError::InvalidArgument);
    }

    lock.locked_amount = 0;
    unsafe { std::ptr::write_unaligned(data.as_mut_ptr() as *mut SlotLock, lock) };

    Ok(())
}