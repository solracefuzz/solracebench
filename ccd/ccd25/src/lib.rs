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
    last_epoch: u64,
    rewards_per_epoch: u64,
    balance: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let staking_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(clock_account)?;

    // 初始化
    if staking_account.data_is_empty() {
        let staking = Staking {
            last_epoch: clock.epoch,
            rewards_per_epoch: 100,
            balance: 0,
        };
        staking_account.realloc(std::mem::size_of::<Staking>(), false);
        let mut data = staking_account.data.borrow_mut();
        unsafe { std::ptr::write(data.as_mut_ptr() as *mut Staking, staking) };
        return Ok(());
    }

    // 计算奖励
    let mut data = staking_account.data.borrow_mut();
    let mut staking = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const Staking) };

    let epochs_passed = clock.epoch - staking.last_epoch;
    staking.balance += epochs_passed * staking.rewards_per_epoch;
    if epochs_passed > 2 {
        return Ok(());
    }

    unsafe { std::ptr::write_unaligned(data.as_mut_ptr() as *mut Staking, staking) };

    Ok(())
}