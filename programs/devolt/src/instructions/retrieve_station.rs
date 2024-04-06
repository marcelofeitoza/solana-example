use anchor_lang::prelude::*;

use crate::state::{CreateBateryReportArgs, Station};

#[derive(Debug, Accounts)]
#[instruction(args: CreateBateryReportArgs)]
pub struct RetrieveStation<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = Station::SPACE, seeds = [Station::PREFIX_SEED.as_ref(), args.id.as_ref()], bump)]
    pub station: Account<'info, Station>,

    pub system_program: Program<'info, System>,
}

pub fn retrieve_station(ctx: Context<RetrieveStation>, _id: String) -> Result<()> {
    let station = &mut ctx.accounts.station;

    msg!("\nINFO: Instruction: Retrieve Station");
    msg!("\nINFO: Station: {:?}", station);

    Ok(())
}