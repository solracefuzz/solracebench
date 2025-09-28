use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use sha2::{Sha256, Digest};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct CryptoAccount {
    value: [u8; 32],
    hash: [u8; 32],
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
        let init_data = CryptoAccount {
            value: [0xFF; 32],
            hash: [0; 32],
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        msg!("Crypto account initialized");
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = CryptoAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // XOR运算
            for byte in &mut data.value {
                *byte ^= 0xAA;
            }
            msg!("XOR applied");
        },
        1 => { // 计算哈希
            let mut hasher = Sha256::new();
            hasher.update(&data.value);
            let result = hasher.finalize();
            data.hash.copy_from_slice(&result);
            msg!("Hash computed");
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}