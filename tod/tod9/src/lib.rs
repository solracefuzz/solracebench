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
struct MatrixAccount {
    matrix: [[i32; 2]; 2],
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
        msg!("Invalid owner");
        return Err(ProgramError::IncorrectProgramId);
    }

    if account.data_len() == 0 {
        let init_data = MatrixAccount {
            matrix: [[1, 2], [3, 4]],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        msg!("Matrix initialized");
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = MatrixAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 转置矩阵
            let mut new_matrix = [[0; 2]; 2];
            for i in 0..2 {
                for j in 0..2 {
                    new_matrix[j][i] = data.matrix[i][j];
                }
            }
            data.matrix = new_matrix;
            msg!("Matrix transposed");
        },
        1 => { // 所有元素加3
            for i in 0..2 {
                for j in 0..2 {
                    data.matrix[i][j] += 3;
                }
            }
            msg!("Elements incremented");
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}