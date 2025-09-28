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
struct AuthAccount {
    flags: u8,
    admin: Pubkey,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if account.data_len() == 0 {
        let init_data = AuthAccount {
            flags: 0b00000000,
            admin: Pubkey::default(),
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        msg!("Auth account initialized");
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = AuthAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 设置管理员
            data.admin = *signer.key;
            data.flags |= 0b00000001; // 设置最低位为管理员标志
            msg!("Admin set");
        },
        1 => { // 切换特权模式（需要管理员权限）
            if data.flags & 0b00000001 != 0 { // 检查管理员标志
                data.flags ^= 0b10000000; // 切换最高位
                msg!("Privilege mode toggled");
            } else {
                msg!("No admin rights");
                return Err(ProgramError::InvalidAccountData);
            }
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
    let datalen = borsh::object_length(&data).unwrap();
    account.realloc(datalen, false);
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}