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
    
    // BUG：epoch值可预测且增长缓慢
    let lottery_number = (clock.epoch % 100) as usize; // 攻击者可提前准备
    select_winner(lottery_number)?;
    Ok(())
}

fn select_winner(lottery_number: usize) -> ProgramResult {
    if lottery_number == 42 {
        msg!("you win!");
    }
    Ok(())
}