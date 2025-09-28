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

const EPOCH_DURATION : i64 = 11100;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let clock = Clock::get()?;
    
    let epoch_end = clock.epoch_start_timestamp + EPOCH_DURATION;
    if clock.unix_timestamp > epoch_end {
        trigger_epoch_end_action()?;
    }
    Ok(())
}

fn trigger_epoch_end_action() -> ProgramResult {
    msg!("log");
    Ok(())
}