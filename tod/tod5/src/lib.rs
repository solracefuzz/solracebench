use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Auction {
    highest_bid: u64,
    is_active: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum AuctionInstruction {
    PlaceBid(u64),
    CloseAuction,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let bidder = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    if !account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // 初始化检测
    if account.data.borrow().is_empty() {
        let init_account = Auction {
            highest_bid: 0,
            is_active: true, // 默认开启拍卖
        };
        let datalen = borsh::object_length(&init_account).unwrap();
        account.realloc(datalen, false);
        init_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
        return Ok(());
    }

    if !bidder.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut auction = Auction::try_from_slice(&account.data.borrow())?;
    let instruction = AuctionInstruction::try_from_slice(instruction_data)?;

    match instruction {
        AuctionInstruction::PlaceBid(amount) => {
            if !auction.is_active {
                return Err(ProgramError::InvalidAccountData);
            }
            if amount > auction.highest_bid {
                auction.highest_bid = amount;
            }
        }
        AuctionInstruction::CloseAuction => {
            auction.is_active = false;
        }
    }

    auction.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}
