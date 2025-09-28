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

const RATE: i64 = 5;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    let clock = Clock::get()?;
    let seed = clock.unix_timestamp as u64; // BUG：可预测的"随机"数
    
    let winner_index = (seed % 100) as usize;
    select_winner(winner_index)?; // 可被提前预测
    Ok(())
}

fn select_winner(winner_index: usize) -> ProgramResult {
    msg!(&format!("{winner_index}").to_string());
    Ok(())
}