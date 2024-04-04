use anchor_lang::prelude::*;

use instructions::*;
use state::*;

mod errors;
mod instructions;
mod state;

declare_id!("7iu9UbDr4B9XSeXP8juRPoD48mKktqWEuk3TmgyVkDHd");

#[program]
pub mod devolt {

    use super::*;

    pub fn batery_report(ctx: Context<CreateBateryReport>, args: CreateBateryReportArgs) -> Result<()> {
        instructions::battery_report(ctx, args)
    }
}
