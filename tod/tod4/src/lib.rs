use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

// 头部导入与示例1相同

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DAO {
    current_owner: Pubkey,
    pending_owner: Option<Pubkey>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum DaoInstruction {
    ProposeOwner(Pubkey),
    ConfirmOwnership,
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
    if !account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // 初始化检测
    if account.data.borrow().is_empty() {
        let init_account = DAO {
            current_owner: *program_id, // 初始所有者设为程序本身
            pending_owner: None,
        };
        let datalen = borsh::object_length(&init_account).unwrap();
        account.realloc(datalen, false);
        init_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let mut dao = DAO::try_from_slice(&account.data.borrow())?;
    let instruction = DaoInstruction::try_from_slice(instruction_data)?;

    if !signer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    match instruction {
        DaoInstruction::ProposeOwner(new_owner) => {
            if *signer.key != dao.current_owner {
                return Err(ProgramError::IllegalOwner);
            }
            dao.pending_owner = Some(new_owner);
        }
        DaoInstruction::ConfirmOwnership => {
            if let Some(pending) = dao.pending_owner {
                if *signer.key != pending {
                    return Err(ProgramError::InvalidArgument);
                }
                dao.current_owner = pending;
                dao.pending_owner = None;
            }
        }
    }
    let datalen = borsh::object_length(&dao).unwrap();
    account.realloc(datalen, false);
    dao.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}