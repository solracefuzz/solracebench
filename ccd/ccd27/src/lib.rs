use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

#[repr(C)]
struct TimeBoundNFT {
    mint_time: i64,
    expiration: i64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let nft_account = next_account_info(accounts_iter)?;
    let client_clock = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(client_clock)?;

    // 初始化NFT
    if nft_account.data_is_empty() {
        let duration = i64::from_le_bytes(instruction_data[..8].try_into().unwrap());
        let nft = TimeBoundNFT {
            mint_time: clock.unix_timestamp,
            expiration: clock.unix_timestamp + duration,
        };
        let datalen = std::mem::size_of::<TimeBoundNFT>();
        nft_account.realloc(datalen, false);
        let mut data = nft_account.data.borrow_mut();
        unsafe { std::ptr::write(data.as_mut_ptr() as *mut TimeBoundNFT, nft) };
        return Ok(());
    }

    // 验证有效性
    let mut data = nft_account.data.borrow_mut();
    let nft = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const TimeBoundNFT) };

    if clock.unix_timestamp > nft.expiration {
        data[16..24].copy_from_slice(&0i64.to_le_bytes());
    }

    Ok(())
}