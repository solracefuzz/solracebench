use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

entrypoint!(process_instruction);

// 状态账户结构
#[repr(C)]
struct ProgramState {
    last_activation_slot: u64,
    activation_timestamp: i64,
    is_active: bool,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let state_account = next_account_info(accounts_iter)?;
    let clock = Clock::get()?;


    if state_account.data_is_empty() {
        let state = ProgramState {
            last_activation_slot: 0,
            activation_timestamp: 0,
            is_active: false,
        };
        let datalen = std::mem::size_of::<ProgramState>();
        state_account.realloc(datalen, false);
        return Ok(());
    }
    // 反序列化状态账户
    let mut state_data = state_account.try_borrow_mut_data()?;
    let state: &mut ProgramState = unsafe {
        &mut *(state_data.as_mut_ptr() as *mut ProgramState)
    };

    // BUG：混合slot和时间戳的竞态条件
    if !state.is_active && 
       clock.slot > state.last_activation_slot + 100 &&
       clock.unix_timestamp > state.activation_timestamp + 3600 
    {
        state.is_active = true;
        msg!("System activated at slot {} timestamp {}", clock.slot, clock.unix_timestamp);
    } else if state.is_active &&
              clock.slot <= state.last_activation_slot + 100 &&
              clock.unix_timestamp <= state.activation_timestamp + 3600 
    {
        state.is_active = false;
        msg!("System deactivated prematurely");
    }

    state.last_activation_slot = clock.slot;
    state.activation_timestamp = clock.unix_timestamp;
    Ok(())
}