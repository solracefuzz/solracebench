
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
struct DataPipeline {
    processed_data: Vec<u8>,
    processing_steps: u8,
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
        let init_data = DataPipeline {
            processed_data: vec![0; 32],
            processing_steps: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = DataPipeline::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 加密操作
            for byte in &mut data.processed_data {
                *byte ^= 0b10101010;
            }
            data.processing_steps |= 0b00000001;
            msg!("XOR encryption applied");
        },
        1 => { // 压缩操作
            data.processed_data = data.processed_data
                .iter()
                .filter(|&&b| b != 0)
                .cloned()
                .collect();
            data.processing_steps |= 0b00000010;
            msg!("Zero-byte compression applied");
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}