use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct PredictionMarketAccount {
    total_pool: u64,
    outcome: Option<bool>,
    bets: [u64; 2], // [支持, 反对]
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if account.data_len() == 0 {
        let init_data = PredictionMarketAccount {
            total_pool: 0,
            outcome: None,
            bets: [0, 0],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = PredictionMarketAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 下注操作
            let amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            let side = instruction_data[9] as usize;
            if side < 2 {
                data.bets[side] += amount;
                data.total_pool += amount;
            }
        },
        1 => { // 解析结果
            if data.outcome.is_none() {
                data.outcome = Some(instruction_data[1] != 0);
                // 计算收益分配逻辑
            }
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}