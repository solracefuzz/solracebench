use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use sha2::{Sha256, Digest};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct ArtAccount {
    pixels: [[u8; 3]; 64], // RGB像素阵列
    generation_steps: u8,
    art_hash: [u8; 32],
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
        let init_data = ArtAccount {
            pixels: [[0; 3]; 64],
            generation_steps: 0,
            art_hash: [0; 32],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = ArtAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 颜色翻转
            for pixel in &mut data.pixels {
                pixel[0] = 255 - pixel[0]; // R
                pixel[1] = 255 - pixel[1]; // G
                pixel[2] = 255 - pixel[2]; // B
            }
            data.generation_steps += 1;
        },
        1 => { // 生成哈希
            let mut hasher = Sha256::new();
            hasher.update(&data.pixels.concat());
            data.art_hash.copy_from_slice(&hasher.finalize());
            data.generation_steps += 1;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}