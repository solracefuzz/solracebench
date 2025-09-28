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
    
    // BUG：该字段与验证者调度相关，不应作为权限判断
    if clock.leader_schedule_epoch == admin_configured_value() { 
        grant_special_access()?; // 可能被恶意验证者操纵
    }
    Ok(())
}

fn admin_configured_value() -> u64 {
    // ...
    std::hint::black_box(42)
}

fn grant_special_access() -> ProgramResult {
    msg!("grant_special_access");
    Ok(())
}