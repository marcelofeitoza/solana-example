mod program_state;
mod models;

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    pubkey::Pubkey,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg
};
use borsh::BorshDeserialize;
use solana_program::account_info::next_account_info;
use program_state::ProgramState;

use models::Action;

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    // if account.owner != program_id {
    //     msg!("Greeted account does not have the correct program id");
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    let mut program_state = ProgramState::new();

    let action = Action::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("\nINFO: Action: {:?}", action);

    match action {
        Action::BatteryReport(report) => program_state.battery_report(report, account),
        Action::PlaceBid(params) => program_state.place_bid(params, account),
    }
}
