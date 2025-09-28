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
struct FlashLoan {
    last_update: i64,
    balance: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let loan_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(clock_account)?;

    // 初始化闪电贷池
    if loan_account.data_is_empty() {
        let pool = FlashLoan {
            last_update: clock.unix_timestamp,
            balance: 1000000,
        };
        let datalen = std::mem::size_of::<FlashLoan>();
        loan_account.realloc(datalen, false);
        let mut data = loan_account.data.borrow_mut();
        unsafe { std::ptr::write(data.as_mut_ptr() as *mut FlashLoan, pool) };
        return Ok(());
    }

    // 处理借款
    let amount = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    let mut data = loan_account.data.borrow_mut();
    let mut pool = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const FlashLoan) };

    let time_elapsed = clock.unix_timestamp - pool.last_update;
    
    if time_elapsed < 60 {
        pool.balance -= amount;
        msg!("Borrowed {} within {} seconds", amount, time_elapsed);
    }

    unsafe { std::ptr::write_unaligned(data.as_mut_ptr() as *mut FlashLoan, pool) };

    Ok(())
}