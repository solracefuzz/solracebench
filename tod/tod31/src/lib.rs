use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct MultisigAccount {
    signers: [Pubkey; 3],
    required: u8,
    signed: [bool; 3],
    executed: bool,
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
        let init_data = MultisigAccount {
            signers: [Pubkey::default(); 3],
            required: 2,
            signed: [false; 3],
            executed: false,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = MultisigAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 添加签名
            if data.executed {
                return Err(ProgramError::Custom(1));
            }
            
            for (i, s) in data.signers.iter().enumerate() {
                if s == signer.key {
                    data.signed[i] = true;
                    break;
                }
            }
        },
        1 => { // 执行交易
            let signed_count = data.signed.iter().filter(|&&s| s).count();
            if signed_count < data.required as usize {
                return Err(ProgramError::Custom(2));
            }
            
            // 执行转账等业务逻辑
            data.executed = true;
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}