use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct SupplyChainAccount {
    production_stage: u8, // 0=原材料 1=生产 2=质检 3=发货
    quality_check: bool,
    history: [String; 2],
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // 验证账户所有权
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // 初始化空账户
    if account.data_len() == 0 {
        let init_data = SupplyChainAccount {
            production_stage: 0,
            quality_check: false,
            history: ["".to_string(), "".to_string()],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = SupplyChainAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 质检操作
            if data.production_stage < 2 {
                data.quality_check = instruction_data[1] != 0;
                data.production_stage = 2;
                data.history[0] = format!("质检结果: {}", data.quality_check);
            }
        },
        1 => { // 发货操作
            if data.production_stage == 2 && data.quality_check {
                data.production_stage = 3;
                data.history[1] = "已发货".to_string();
            } else {
                data.history[1] = "未通过质检发货".to_string();
            }
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}