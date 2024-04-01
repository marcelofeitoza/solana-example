use std::{collections::HashMap};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    clock::Clock,
    sysvar::Sysvar,
};

use crate::models::{Bid, Station, BatteryReportParams, PlaceBidParams};

pub struct ProgramState {
    pub stations: HashMap<String, Station>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            stations: HashMap::<String, Station>::new(),
        }
    }

    pub fn battery_report(&mut self, report: BatteryReportParams, account: &AccountInfo) -> ProgramResult {
        msg!("\nINFO: Instruction: Battery Report");
        msg!("\nINFO: Report: {:?}", report);
        msg!("\nINFO: Account: {:?}", account);

        msg!("\nINFO: Stations: {:?}", &self.stations);

        let station: &mut Station = self.stations.entry(report.id.clone()).or_insert(Station {
            id: report.id.clone(),
            latitude: report.latitude,
            longitude: report.longitude,
            max_capacity: report.max_capacity,
            battery_level: report.battery_level,
            auction: None,
        });
        msg!("\nINFO: Station with ID: {} created or updated", report.id);

        station.latitude = report.latitude;
        station.longitude = report.longitude;
        station.max_capacity = report.max_capacity;
        station.battery_level = report.battery_level;

        msg!("\nINFO: Battery report received for station with ID: {}", report.id);

        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;

        if let Some(auction) = &mut station.auction {
            if auction.ongoing {
                let clock = Clock::get()?;
                let current_timestamp = clock.unix_timestamp as u64;
                if auction.timestamp <= current_timestamp {
                    station.finalize_auction(current_timestamp);
                    msg!("\nINFO: Auction finalized for station with ID: {}", report.id);
                }
            }
        } else {
            let battery_deficit = station.max_capacity * (station.battery_level / 100.0);
            let req_charge = station.max_capacity - battery_deficit;
            if req_charge >= 20.0 {
                if let Some(auction) = &mut station.auction {
                    if !auction.ongoing {
                        station.create_auction(req_charge, current_timestamp);
                    }
                } else {
                    station.create_auction(req_charge, current_timestamp);
                }
            }
        }

        msg!("Stations: {:?}", &self.stations);
        Ok(())
    }

    pub fn place_bid(&mut self, params: PlaceBidParams, account: &AccountInfo) -> ProgramResult {
        let station_id = params.id.clone();
        let bidder = params.bidder.clone();
        let amount = params.amount.clone();
        let price_per_amount = params.price_per_amount.clone();

        msg!("\nINFO: Instruction: Place Bid");
        msg!("\nINFO: Params: {:?}", params);
        msg!("\nINFO: Account: {:?}", account);

        if let Some(station) = self.stations.get_mut(&station_id) {
            if let Some(auction) = &mut station.auction {
                if auction.ongoing {
                    let clock = Clock::get()?;
                    let current_timestamp = clock.unix_timestamp as u64;
                    if auction.timestamp <= current_timestamp {
                        msg!("\nINFO: Placing bid for auction in station with ID: {}", station_id);
                        let bid = Bid {
                            bidder,
                            amount,
                            price_per_amount,
                        };
                        auction.bids.push(bid);
                    } else {
                        msg!("\nERROR: Auction for station with ID: {} has already ended.", station_id);
                    }
                } else {
                    msg!("\nERROR: No ongoing auction for station with ID: {}", station_id);
                }
            } else {
                msg!("\nERROR: No auction found for station with ID: {}", station_id);
            }
        } else {
            msg!("\nERROR: No station found with ID: {}", station_id);
        }

        Ok(())
    }
}