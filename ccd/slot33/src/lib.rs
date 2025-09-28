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
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let clock = Clock::get()?;
    
    let slot_duration_seconds = clock.slot * 400 / 1000;
    if slot_duration_seconds > 3600 {
        activate_feature()?;
    }
    Ok(())
}

fn activate_feature() -> ProgramResult {
    msg!("log");
    Ok(())
}