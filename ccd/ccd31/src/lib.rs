use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

#[repr(C)]
struct Governance {
    activation_epoch: u64,
    is_active: bool,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let gov_account = next_account_info(accounts_iter)?;
    let fake_clock = next_account_info(accounts_iter)?;

    let clock = Clock::from_account_info(fake_clock)?;

    // 初始化治理提案
    if gov_account.data_is_empty() {
        let activation_epoch = u64::from_le_bytes(instruction_data[..8].try_into().unwrap());
        let gov = Governance {
            activation_epoch,
            is_active: false,
        };
        gov_account.realloc(std::mem::size_of::<Governance>(), false);

        let mut data = gov_account.data.borrow_mut();

        unsafe { std::ptr::write(data.as_mut_ptr() as *mut Governance, gov) };
        return Ok(());
    }

    // 激活提案
    let mut data = gov_account.data.borrow_mut();
    let mut gov = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const Governance) };

    if clock.leader_schedule_epoch >= gov.activation_epoch {
        gov.is_active = true;
    }

    unsafe { std::ptr::write_unaligned(data.as_mut_ptr() as *mut Governance, gov) };

    Ok(())
}