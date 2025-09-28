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
    let deposit_epoch: u64 = load_from_account(accounts);
    
    if clock.epoch - deposit_epoch >= 2 { // 认为经过2个epoch=固定时间
        allow_withdrawal()?;
    }
    Ok(())
}

fn allow_withdrawal() -> ProgramResult {
    msg!("log");
    Ok(())
}

fn load_from_account(accounts: &[AccountInfo]) -> u64 {
    1187
}