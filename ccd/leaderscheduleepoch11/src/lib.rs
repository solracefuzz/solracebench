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

    if clock.leader_schedule_epoch % 2 == 0 { // 奇偶epoch切换
        transfer_admin_powers()?; // 攻击者可能预测切换时间
    }
    Ok(())
}

fn transfer_admin_powers() -> ProgramResult {
    msg!("log");
    Ok(())
}