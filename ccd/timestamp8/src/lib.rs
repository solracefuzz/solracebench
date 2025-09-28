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
    let last_claim_time: i64 = load_from_account();
    let current_time = clock.unix_timestamp;

    let elapsed = current_time - last_claim_time;
    let reward = elapsed * RATE;
    
    save_to_account(current_time)?;
    Ok(())
}

fn load_from_account() -> i64 {
    42
}

fn save_to_account(x: i64) -> ProgramResult {
    if x % 10 == 0 {
        msg!("log");
    }
    Ok(())
}
