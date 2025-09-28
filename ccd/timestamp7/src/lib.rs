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
    let local_offset = 8 * 3600;
    let target = (1638316800 - local_offset) as i64;
    
    if clock.unix_timestamp > target {
        msg!("log");
    }
    Ok(())
}
