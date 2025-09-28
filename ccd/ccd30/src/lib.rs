use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

#[repr(C)]
struct InterestAccount {
    last_slot: u64,
    principal: u64,
    rate: u64, // 每slot利率
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let interest_account = next_account_info(accounts_iter)?;
    let fake_clock = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(fake_clock)?;

    // 初始化利息账户
    if interest_account.data_is_empty() {
        let rate = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
        let account = InterestAccount {
            last_slot: clock.slot,
            principal: 0,
            rate,
        };
        let datalen = std::mem::size_of::<InterestAccount>();
        interest_account.realloc(datalen, false);
        let mut data = interest_account.data.borrow_mut();
        unsafe { std::ptr::write(data.as_mut_ptr() as *mut InterestAccount, account) };
        return Ok(());
    }

    // 计算复利
    let mut data = interest_account.data.borrow_mut();
    let mut account = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const InterestAccount) };

    let slots_elapsed = clock.slot - account.last_slot;
    let interest = account.principal * account.rate * slots_elapsed / 10000;

    if slots_elapsed > 100 {
        account.principal += interest;
        account.last_slot = clock.slot;
    }

    unsafe { std::ptr::write_unaligned(data.as_mut_ptr() as *mut InterestAccount, account) };

    Ok(())
}