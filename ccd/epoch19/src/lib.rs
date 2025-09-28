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

// 账户数据布局
#[derive(Debug)]
struct VaultAccount {
    locked_until_epoch: u64,
    amount: u64,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let vault_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let clock = Clock::get()?;


    if vault_account.data_is_empty() {
        let vault = VaultAccount {
            locked_until_epoch: 100,
            amount: 3,
        };
        let datalen = std::mem::size_of::<VaultAccount>();
        vault_account.realloc(datalen, false);
        return Ok(());
    }

    // 反序列化金库账户
    let mut vault_data = vault_account.try_borrow_mut_data()?;
    let vault: &mut VaultAccount = unsafe {
        &mut *(vault_data.as_mut_ptr() as *mut VaultAccount)
    };

    // BUG：直接比较当前epoch与存储epoch
    if clock.epoch <= vault.locked_until_epoch {
        msg!("Funds still locked until epoch {}", vault.locked_until_epoch);
        return Err(ProgramError::InvalidArgument);
    }

    // 转账逻辑
    let transfer_amount = vault.amount;
    **vault_account.lamports.borrow_mut() -= transfer_amount;
    **user_account.lamports.borrow_mut() += transfer_amount;

    vault.amount = 0; // 清空金库
    Ok(())
}