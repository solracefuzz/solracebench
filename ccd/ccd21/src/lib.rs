use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{
        clock::Clock,
        Sysvar,
    },
    msg,
    system_instruction,
    program::invoke,
};

entrypoint!(process_instruction);

// 竞拍数据结构
#[repr(C)]
#[derive(Debug)]
struct AuctionData {
    end_slot: u64,
    highest_bid: u64,
    bidder: [u8; 32],
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    
    // 账户顺序：
    // 0. 支付账户
    // 1. 竞拍账户
    // 2. 出价者账户
    // 3. 系统账户
    // 4. Clock sysvar
    
    let payer = next_account_info(account_iter)?;
    let auction_account = next_account_info(account_iter)?;
    let bidder = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;
    let clock_account = next_account_info(account_iter)?;

    let clock = Clock::from_account_info(clock_account)?;
    
    // 初始化竞拍账户
    if auction_account.data_is_empty() {
        let client_end_slot = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());

        let auction_data = AuctionData {
            end_slot: client_end_slot,
            highest_bid: 0,
            bidder: [0; 32],
        };
        let datalen = std::mem::size_of::<AuctionData>();
        auction_account.realloc(datalen, false);
        let mut data = auction_account.data.borrow_mut();

        // 序列化到账户数据
        let mut buffer = vec![0; std::mem::size_of::<AuctionData>()];
        buffer[..8].copy_from_slice(&auction_data.end_slot.to_le_bytes());
        buffer[8..16].copy_from_slice(&auction_data.highest_bid.to_le_bytes());
        buffer[16..48].copy_from_slice(&auction_data.bidder);
        data.copy_from_slice(&buffer);
        return Ok(());
    }

    // 处理出价逻辑
    let current_slot = clock.slot;
    let mut data = auction_account.data.borrow_mut();
    let mut auction_data = unsafe {
        std::ptr::read_unaligned(data.as_ptr() as *const AuctionData)
    };

    if current_slot > auction_data.end_slot {
        msg!("Auction already ended");
        return Err(ProgramError::InvalidInstructionData);
    }

    let new_bid = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
    
    // 转账逻辑
    let transfer_instruction = system_instruction::transfer(
        bidder.key,
        auction_account.key,
        new_bid,
    );
    
    invoke(
        &transfer_instruction,
        &[
            bidder.clone(),
            auction_account.clone(),
            system_program.clone(),
        ],
    )?;

    // 更新最高出价
    if new_bid > auction_data.highest_bid {
        auction_data.highest_bid = new_bid;
        auction_data.bidder.copy_from_slice(bidder.key.as_ref());
    }

    // 回写数据
    unsafe {
        std::ptr::write_unaligned(
            data.as_mut_ptr() as *mut AuctionData,
            auction_data
        );
    }

    Ok(())
}
