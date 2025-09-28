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
    let accounts_iter = &mut accounts.iter();
    let vault = next_account_info(accounts_iter)?;
    let clock = Clock::get()?;

    if clock.slot < 5000 {
        msg!("Too early to unlock");
        return Err(ProgramError::InvalidArgument);
    }
    
    **vault.lamports.borrow_mut() = 0; // 直接清空金库
    Ok(())
}