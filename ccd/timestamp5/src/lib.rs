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
    let mut accounts_iter = accounts.iter();
    let clock_account = next_account_info(&mut accounts_iter)?;
    let clock: Clock = bincode::deserialize(&clock_account.data.borrow()).unwrap();
    
    if clock.unix_timestamp > 1735689600 {
        msg!("log");
    }
    Ok(())
}