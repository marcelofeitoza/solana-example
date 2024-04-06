use anchor_lang::prelude::*;

use instructions::*;
use state::*;

mod errors;
mod instructions;
mod state;

declare_id!("EWQ3cdo7pHZnmuubzDMAdso5U8nAwbhrAeSKuEw9R2mg");

#[program]
pub mod devolt {

    use super::*;

    pub fn retrieve_station(ctx: Context<RetrieveStation>, id: String) -> Result<()> {
        instructions::retrieve_station(ctx, id)
    }

    pub fn battery_report(ctx: Context<CreateBateryReport>, args: CreateBateryReportArgs) -> Result<()> {
        instructions::battery_report(ctx, args)
    }

    pub fn place_bid(ctx: Context<PlaceBid>, args: PlaceBidArgs) -> Result<()> {
        instructions::place_bid(ctx, args)
    }
}
