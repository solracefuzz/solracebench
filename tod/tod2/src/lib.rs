use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Wallet {
    pub balance: u64,
    fee_rate: u8,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum WalletInstruction {
    SetFee(u8),
    Transfer(u64),
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // 验证账户所有者
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // 初始化检测
    if account.data.borrow().is_empty() {
        let init_account = Wallet {
            balance: 0,
            fee_rate: 0,
        };
        let datalen = borsh::object_length(&init_account).unwrap();
        account.realloc(datalen, false);
        init_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let mut wallet = Wallet::try_from_slice(&account.data.borrow())?;
    let instruction = WalletInstruction::try_from_slice(instruction_data)?;

    match instruction {
        WalletInstruction::SetFee(rate) => {
            wallet.fee_rate = rate.min(100);
        }
        WalletInstruction::Transfer(amount) => {
            let fee = amount.checked_mul(wallet.fee_rate as u64)
                .and_then(|v| v.checked_div(100))
                .ok_or(ProgramError::ArithmeticOverflow)?;
            
            wallet.balance = wallet.balance
                .checked_sub(amount)
                .and_then(|v| v.checked_sub(fee))
                .ok_or(ProgramError::InsufficientFunds)?;
        }
    }

    wallet.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}