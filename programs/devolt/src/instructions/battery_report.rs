use anchor_lang::prelude::*;

use crate::state::{CreateBateryReportArgs, Station};

#[derive(Debug, Accounts)]
#[instruction(args: CreateBateryReportArgs)]
pub struct CreateBateryReport<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // #[account(init, payer = signer, space = Station::SPACE, seeds = [Station::PREFIX_SEED.as_ref(), args.id.as_ref()], bump)]
    #[account(init_if_needed, payer = signer, space = Station::SPACE, seeds = [Station::PREFIX_SEED.as_ref(), args.id.as_ref()], bump)]
    pub station: Account<'info, Station>,

    pub system_program: Program<'info, System>,
}

pub fn battery_report(
    ctx: Context<CreateBateryReport>,
    args: CreateBateryReportArgs,
) -> Result<()> {
    let station: &mut Account<Station> = &mut ctx.accounts.station;

    msg!("\nINFO: Instruction: Battery Report");

    station.auth = *ctx.accounts.signer.key;
    station.id = args.id;
    station.latitude = args.latitude;
    station.longitude = args.longitude;
    station.max_capacity = args.max_capacity;
    station.battery_level = args.battery_level;

    let clock = Clock::get().unwrap();
    let _current_timestamp = clock.unix_timestamp as u64;

    if let Some(auction) = &mut station.auction {
        if auction.ongoing {
            let clock = Clock::get().unwrap();
            let current_timestamp = clock.unix_timestamp as u64;
            if auction.timestamp <= current_timestamp {
                msg!(
                    "\nINFO: Auction finalized for station with ID: {}",
                    station.id
                );
                // station.finalize_auction(current_timestamp);
            }
        }
    } else {
        let battery_deficit = station.max_capacity * (station.battery_level / 100.0);
        let req_charge = station.max_capacity - battery_deficit;
        if req_charge >= 20.0 {
            if let Some(auction) = &mut station.auction {
                if !auction.ongoing {
                    msg!("Creating auction for {}", station.id);
                    // station.create_auction(req_charge, current_timestamp);
                }
            } else {
                msg!("Creating auction for {}", station.id);
                // station.create_auction(req_charge, current_timestamp);
            }
        }
    }

    msg!("\nINFO: Station: {:?}", station);

    // station.exit(&ctx.program_id)?;

    Ok(())

    // pub fn battery_report(
    //     &mut self,
    //     report: BatteryReportParams,
    //     account: &AccountInfo,
    // ) -> ProgramResult {
    //     msg!("\nINFO: Instruction: Battery Report");
    //     msg!("\nINFO: Report: {:?}", report);
    //     msg!("\nINFO: Account: {:?}", account);

    //     msg!("\nINFO: Stations: {:?}", &self.stations);

    //     let station: &mut Station = self.stations.entry(report.id.clone()).or_insert(Station {
    //         id: report.id.clone(),
    //         latitude: report.latitude,
    //         longitude: report.longitude,
    //         max_capacity: report.max_capacity,
    //         battery_level: report.battery_level,
    //         auction: None,
    //     });
    //     msg!("\nINFO: Station with ID: {} created or updated", report.id);

    //     station.latitude = report.latitude;
    //     station.longitude = report.longitude;
    //     station.max_capacity = report.max_capacity;
    //     station.battery_level = report.battery_level;

    //     msg!(
    //         "\nINFO: Battery report received for station with ID: {}",
    //         report.id
    //     );

    //     let clock = Clock::get().unwrap();
    //     let current_timestamp = clock.unix_timestamp as u64;

    //     if let Some(auction) = &mut station.auction {
    //         if auction.ongoing {
    //             let clock = Clock::get().unwrap();
    //             let current_timestamp = clock.unix_timestamp as u64;
    //             if auction.timestamp <= current_timestamp {
    //                 station.finalize_auction(current_timestamp);
    //                 msg!(
    //                     "\nINFO: Auction finalized for station with ID: {}",
    //                     report.id
    //                 );
    //             }
    //         }
    //     } else {
    //         let battery_deficit = station.max_capacity * (station.battery_level / 100.0);
    //         let req_charge = station.max_capacity - battery_deficit;
    //         if req_charge >= 20.0 {
    //             if let Some(auction) = &mut station.auction {
    //                 if !auction.ongoing {
    //                     station.create_auction(req_charge, current_timestamp);
    //                 }
    //             } else {
    //                 station.create_auction(req_charge, current_timestamp);
    //             }
    //         }
    //     }

    //     // account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    //     msg!("Stations: {:?}", &self.stations);
    //     Ok(())
    // }

    // pub fn finalize_auction(&mut self, timestamp: u64) {
    //     msg!("Finalizing auction for station with ID: {}", self.id);

    //     if let Some(auction) = &mut self.auction {
    //         msg!("Auction ongoing for station with ID: {}", self.id);

    //         if auction.ongoing && auction.timestamp <= timestamp {
    //             msg!("Auction ongoing for station with ID: {}", self.id);

    //             let mut accepted_bids: Vec<Bid> = Vec::new();
    //             let mut energy_needed = auction.req_charge;

    //             auction
    //                 .bids
    //                 .sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());

    //             for bid in &auction.bids {
    //                 msg!("Bidder: {}, Amount: {}", bid.bidder, bid.amount);

    //                 if energy_needed <= 0.0 {
    //                     msg!("Energy needed is 0.0");
    //                     break;
    //                 }

    //                 if bid.amount <= energy_needed {
    //                     msg!("Bid amount is less than energy needed");

    //                     let bid = Bid {
    //                         bidder: bid.bidder.clone(),
    //                         amount: bid.amount,
    //                         price_per_amount: bid.price_per_amount,
    //                     };
    //                     accepted_bids.push(bid.clone());
    //                     energy_needed -= bid.amount;
    //                 } else {
    //                     msg!("Bid amount is more than energy needed");

    //                     let partial_bid = Bid {
    //                         bidder: bid.bidder.clone(),
    //                         amount: energy_needed,
    //                         price_per_amount: bid.price_per_amount,
    //                     };
    //                     accepted_bids.push(partial_bid);
    //                     energy_needed = 0.0;
    //                 }
    //             }

    //             for accepted_bid in &accepted_bids {
    //                 let accepted_bid = Bid {
    //                     bidder: accepted_bid.bidder.clone(),
    //                     amount: accepted_bid.amount,
    //                     price_per_amount: accepted_bid.price_per_amount,
    //                 };

    //                 msg!(
    //                     "Accepted bid: Bidder: {}, Amount: {}",
    //                     accepted_bid.bidder.clone(),
    //                     accepted_bid.amount.clone()
    //                 );

    //                 self.battery_level += accepted_bid.amount;
    //                 auction.winning_bids.push(accepted_bid);
    //             }

    //             msg!("Auction finalized for station with ID: {}", self.id);
    //             auction.ongoing = false;
    //         }
    //     }
    // }
}

