use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct ModelAccount {
    weights: [f32; 3],
    learning_rate: f32,
    update_count: u32,
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
        let init_data = ModelAccount {
            weights: [0.5; 3],
            learning_rate: 0.01,
            update_count: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = ModelAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 梯度更新
            let gradients: [f32; 3] = instruction_data[1..13]
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            
            for (w, g) in data.weights.iter_mut().zip(gradients.iter()) {
                *w -= data.learning_rate * g;
            }
            data.update_count += 1;
        },
        1 => { // 学习率衰减
            data.learning_rate *= 0.9;
            data.update_count += 1;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}