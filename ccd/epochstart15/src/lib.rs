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

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let clock = Clock::get()?;
    let last_epoch_start = load_from_account(accounts);
    
    if clock.epoch_start_timestamp != last_epoch_start {
        update_state(accounts)?;
    }
    Ok(())
}

fn load_from_account(accounts: &[AccountInfo]) -> i64 {
    333
}

fn update_state(accounts: &[AccountInfo]) -> ProgramResult {
    msg!("log");
    Ok(())
}