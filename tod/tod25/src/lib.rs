use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MusicAccount {
    notes: Vec<u8>,
    effects: u8,
    generation: u32,
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
        let init_data = MusicAccount {
            notes: vec![60; 8], // 初始C4音符
            effects: 0b00000000,
            generation: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = MusicAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 添加音符
            let new_note = instruction_data[1];
            data.notes.push(new_note);
            data.generation += 1;
        },
        1 => { // 应用效果
            let effect_mask = instruction_data[1];
            data.effects |= effect_mask;
            
            if effect_mask & 0b00000001 != 0 { // 移调效果
                for note in &mut data.notes {
                    *note = note.saturating_add(12);
                }
            }
            if effect_mask & 0b00000010 != 0 { // 反转序列
                data.notes.reverse();
            }
            data.generation += 1;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}