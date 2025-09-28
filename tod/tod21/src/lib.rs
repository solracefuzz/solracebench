use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct BiometricAccount {
    template: Vec<u8>,
    verification_threshold: f32,
    access_log: [u64; 2],
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
        let init_data = BiometricAccount {
            template: vec![0; 256],
            verification_threshold: 0.85,
            access_log: [0; 2],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = BiometricAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 更新模板
            let new_template = instruction_data[1..].to_vec();
            if new_template.len() == 256 {
                data.template = new_template;
                data.access_log[0] += 1;
            }
        },
        1 => { // 调整阈值
            let new_threshold = f32::from_le_bytes(instruction_data[1..5].try_into().unwrap());
            data.verification_threshold = new_threshold.clamp(0.0, 1.0);
            data.access_log[1] += 1;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}