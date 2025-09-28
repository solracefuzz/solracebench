use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct RenderAccount {
    vertices: [[f32; 3]; 4],
    transform_matrix: [[f32; 4]; 4],
    render_count: u32,
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
        let init_data = RenderAccount {
            vertices: [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0]
            ],
            transform_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ],
            render_count: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = RenderAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 顶点变换
            for vertex in &mut data.vertices {
                let mut temp = [0.0; 4];
                for i in 0..4 {
                    temp[i] = data.transform_matrix[i][0] * vertex[0] +
                             data.transform_matrix[i][1] * vertex[1] +
                             data.transform_matrix[i][2] * vertex[2] +
                             data.transform_matrix[i][3];
                }
                vertex[0] = temp[0];
                vertex[1] = temp[1];
                vertex[2] = temp[2];
            }
            data.render_count += 1;
        },
        1 => { // 矩阵更新
            let new_matrix: [[f32; 4]; 4] = instruction_data[1..]
                .chunks_exact(16)
                .next()
                .map(|chunk| {
                    let mut matrix = [[0.0; 4]; 4];
                    for (i, row) in chunk.chunks_exact(4).enumerate() {
                        for (j, &byte) in row.iter().enumerate() {
                            matrix[i][j] = f32::from_le_bytes([byte, 0, 0, 0]);
                        }
                    }
                    matrix
                })
                .unwrap_or(data.transform_matrix);
            data.transform_matrix = new_matrix;
            data.render_count += 1;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}