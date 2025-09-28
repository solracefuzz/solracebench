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
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let user_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let clock = Clock::get()?;

    // 假设我们在区块高度1000之后允许提款
    let target_block_height = 1000;

    if clock.slot < target_block_height {
        msg!("Withdrawal not allowed until block height {}", target_block_height);
        return Err(ProgramError::InvalidArgument);
    }

    // 提款逻辑
    let user_lamports = user_account.lamports();
    let vault_lamports = vault_account.lamports();

    **user_account.lamports.borrow_mut() = user_lamports.checked_add(vault_lamports).ok_or(ProgramError::InsufficientFunds)?;
    **vault_account.lamports.borrow_mut() = 0;

    msg!("Withdrawal successful!");

    Ok(())
}