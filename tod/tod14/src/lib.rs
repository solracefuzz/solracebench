use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct GameState {
    position: (i32, i32),
    health: u8,
    status: String,
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
        let init_data = GameState {
            position: (0, 0),
            health: 100,
            status: "active".to_string(),
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = GameState::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 移动操作
            let x = i32::from_le_bytes(instruction_data[1..5].try_into().unwrap());
            let y = i32::from_le_bytes(instruction_data[5..9].try_into().unwrap());
            data.position.0 = data.position.0.saturating_add(x);
            data.position.1 = data.position.1.saturating_add(y);
            data.status = format!("Moved to ({}, {})", data.position.0, data.position.1);
        },
        1 => { // 攻击操作
            if data.position.0 > 10 {
                data.health = data.health.saturating_sub(30);
                data.status = "High ground attack".to_string();
            } else {
                data.health = data.health.saturating_sub(10);
                data.status = "Normal attack".to_string();
            }
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}