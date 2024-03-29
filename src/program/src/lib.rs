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
    StartAuction,
    MakeBid(u64),
    FinalizeAuction,
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
        Action::StartAuction => start_auction(accounts),
        Action::MakeBid(amount) => place_bid(accounts, amount),
        Action::FinalizeAuction => finalize_auction(accounts),
    }
}

fn start_auction(_accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Instruction: Start Auction");
    Ok(())
}

fn place_bid(_accounts: &[AccountInfo], _amount: u64) -> ProgramResult {  
    msg!("Instruction: Place Bid");
    Ok(())
}

fn finalize_auction(_accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Instruction: Finalize Auction");
    Ok(())
}