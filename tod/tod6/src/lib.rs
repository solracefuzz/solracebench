use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// 定义数据结构
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DataAccount {
    pub value: u64,
    pub last_operation: String,
}

// 声明程序入口点
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 解析指令类型（0或1）
    let instruction = instruction_data[0];
    
    // 获取账户迭代器
    let accounts_iter = &mut accounts.iter();
    
    // 获取目标账户
    let account = next_account_info(accounts_iter)?;

    // 验证程序拥有该账户
    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // 初始化检查（数据长度为0时初始化）
    if account.data_len() == 0 {
        msg!("Initializing account");
        let mut data = DataAccount {
            value: 0,
            last_operation: "None".to_string(),
        };
        let datalen = borsh::object_length(&data).unwrap();
        account.realloc(datalen, false);
        data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    }

    // 反序列化现有数据
    let mut data_account = DataAccount::try_from_slice(&account.data.borrow())?;

    // 根据指令类型处理不同操作
    match instruction {
        0 => {  // 操作A：先加后记录
            data_account.value = data_account.value.wrapping_add(10);
            data_account.last_operation = "OperationA".to_string();
            msg!("Executed Operation A");
        },
        1 => {  // 操作B：先乘后记录
            data_account.value = data_account.value.wrapping_mul(2);
            data_account.last_operation = "OperationB".to_string();
            msg!("Executed Operation B");
        },
        _ => {
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidInstructionData)
        }
    }

    // 序列化并保存修改后的数据
    let datalen = borsh::object_length(&data_account).unwrap();
    account.realloc(datalen, false);
    data_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}