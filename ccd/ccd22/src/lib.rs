use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
    msg,
};

// 定义拍卖数据结构
#[repr(C)]
#[derive(Debug)]
struct Auction {
    end_time: i64,
    highest_bid: u64,
    bidder: [u8; 32],
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    // 账户顺序：
    // 0. 拍卖账户
    // 1. 出价者账户
    // 2. 支付账户
    // 3. Clock sysvar
    
    let auction_account = next_account_info(accounts_iter)?;
    let bidder_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(clock_account)?;

    // 初始化拍卖账户
    if auction_account.data_is_empty() {
        if !payer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let duration_seconds = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
        let end_time = clock.unix_timestamp + duration_seconds as i64;
        
        let auction = Auction {
            end_time,
            highest_bid: 0,
            bidder: [0; 32],
        };
        let datalen = std::mem::size_of::<Auction>();
        auction_account.realloc(datalen, false);
        let mut data = auction_account.data.borrow_mut();

        unsafe {
            std::ptr::write_unaligned(
                data.as_mut_ptr() as *mut Auction,
                auction
            );
        }
        return Ok(());
    }

    // 处理出价
    let bid_amount = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    
    let mut data = auction_account.data.borrow_mut();
    let mut auction = unsafe {
        std::ptr::read_unaligned(data.as_ptr() as *const Auction)
    };

    if clock.unix_timestamp > auction.end_time {
        msg!("Auction expired");
        return Err(ProgramError::InvalidArgument);
    }

    if bid_amount > auction.highest_bid {
        auction.highest_bid = bid_amount;
        auction.bidder.copy_from_slice(bidder_account.key.as_ref());
    }

    unsafe {
        std::ptr::write_unaligned(
            data.as_mut_ptr() as *mut Auction,
            auction
        );
    }

    Ok(())
}