use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct AuctionAccount {
    highest_bid: u64,
    reserve_price: u64,
    bid_count: u32,
    status: u8, // 0=进行中 1=结束
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
        let init_data = AuctionAccount {
            highest_bid: 0,
            reserve_price: 1000,
            bid_count: 0,
            status: 0,
        };
        let datalen = borsh::object_length(&init_data).unwrap();
        account.realloc(datalen, false);
        init_data.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    let instruction = instruction_data[0];
    let mut data = AuctionAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        0 => { // 出价操作
            if data.status == 0 {
                let bid = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
                if bid > data.highest_bid && bid >= data.reserve_price {
                    data.highest_bid = bid;
                    data.bid_count += 1;
                }
            }
        },
        1 => { // 修改保留价
            let new_price = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
            data.reserve_price = new_price;
            // 如果当前最高价低于新保留价则重置
            if data.highest_bid < data.reserve_price {
                data.highest_bid = 0;
                data.bid_count = 0;
            }
        },
        _ => return Err(ProgramError::InvalidInstructionData)
    }

    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}