use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Action {
    BatteryReport(u64),
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let action = Action::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match action {
        Action::BatteryReport(amount) => battery_report(accounts, amount),
    }
}

fn battery_report(_accounts: &[AccountInfo], amount: u64) -> ProgramResult {  
    msg!("Instruction: Place Bid: {}", amount);
    Ok(())
}