mod program_state;
mod models;

use solana_program::{account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey, program_error::ProgramError, msg};
use borsh::BorshDeserialize;
use program_state::ProgramState;
use models::{Action};

entrypoint!(process_instruction);

fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let mut program_state = ProgramState::new();

    let action = Action::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match action {
        Action::BatteryReport(report) => program_state.battery_report(report, accounts),
        Action::PlaceBid(amount) => program_state.place_bid(amount, accounts),
        Action::StartAuction => program_state.get_stations(accounts),
        Action::FinalizeAuction => finalize_auction(accounts),
    }
}

fn finalize_auction(_accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Instruction: Finalize Auction");
    Ok(())
}