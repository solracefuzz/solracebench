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
    let start = 1638316800;
    let end = start + 3600; // 1小时窗口
    
    if (clock.unix_timestamp >= start) && (clock.unix_timestamp <= end) {
        distribute_rewards(accounts)?;
        msg!("log");
    }
    Ok(())
}

pub fn distribute_rewards(accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}