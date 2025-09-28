use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct LogisticsAccount {
    locations: [String; 2],
    status: u8, // 0=待发 1=运输中 2=已签收
    verification_code: u32,
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
        let init_data = LogisticsAccount {
            locations: ["".to_string(), "".to_string()],
            status: 0,
            verification_code: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = LogisticsAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 更新位置
            let location = String::from_utf8(instruction_data[1..].to_vec())
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            data.locations[0] = data.locations[1].clone();
            data.locations[1] = location;
        },
        1 => { // 状态验证
            let code = u32::from_le_bytes(instruction_data[1..5].try_into().unwrap());
            if code == data.verification_code {
                data.status = data.status.saturating_add(1);
            }
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}