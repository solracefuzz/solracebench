use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

// 定义账户数据结构
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BankAccount {
    pub balance: u64,
}

// 定义支持的指令
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum BankInstruction {
    Increment,
    Double,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 解析指令
    let instruction = BankInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // 获取账户迭代器
    let accounts_iter = &mut accounts.iter();

    // 获取目标账户
    let account = next_account_info(accounts_iter)?;

    // 验证账户可写
    if !account.is_writable {
        msg!("Account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if account.data_len() == 0 {
        let init_data = BankAccount {
            balance: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        msg!("Account initialized");
        return Ok(());
    }
    // 反序列化账户数据
    let mut bank_account = BankAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        BankInstruction::Increment => {
            msg!("Executing Increment instruction");
            bank_account.balance = bank_account.balance.checked_add(1)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        }
        BankInstruction::Double => {
            msg!("Executing Double instruction");
            bank_account.balance = bank_account.balance.checked_mul(2)
                .ok_or(ProgramError::ArithmeticOverflow)?;
        }
    }

    // 序列化并保存修改后的数据
    bank_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
