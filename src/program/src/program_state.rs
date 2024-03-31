use std::{collections::HashMap};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    clock::Clock,
    sysvar::Sysvar,
};


use crate::models::Station;
use crate::models::BatteryReport;

pub struct ProgramState {
    stations: HashMap<String, Station>,
}

impl ProgramState {
    pub fn new() -> Self {
        Self {
            stations: HashMap::new(),
        }
    }

    pub fn get_stations(&self, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Stations: {:?}", self.stations);
        msg!("Accounts: {:?}", accounts);

        Ok(())
    }

    pub fn battery_report(&mut self, report: BatteryReport, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Instruction: Battery Report");
        msg!("Report: {:?}", report);
        msg!("Accounts: {:?}", accounts);

        let station: &mut Station = self.stations.entry(report.id.clone()).or_insert(Station {
            id: report.id.clone(),
            latitude: report.latitude,
            longitude: report.longitude,
            max_capacity: report.max_capacity,
            battery_level: report.battery_level,
            auction: None,
        });
        msg!("Station with ID: {} created or updated", report.id);

        station.latitude = report.latitude;
        station.longitude = report.longitude;
        station.max_capacity = report.max_capacity;
        station.battery_level = report.battery_level;

        msg!("Battery report received for station with ID: {}", report.id);

        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp as u64;

        if let Some(auction) = &station.auction {
            msg!("Auction ongoing for station with ID: {}", report.id);

            if auction.ongoing {
                msg!("Auction ongoing for station with ID: {}", report.id);

                if auction.timestamp <= current_timestamp {
                    station.finalize_auction(current_timestamp);
                    msg!("Auction finalized for station with ID: {}", report.id);
                }
            }
        } else {
            msg!("Auction not ongoing for station with ID: {}", report.id);

            let battery_deficit = station.max_capacity * (station.battery_level / 100.0);
            let req_charge = station.max_capacity - battery_deficit;
            if req_charge >= 20.0 {
                msg!("Creating auction for station with ID: {}", report.id);
                station.create_auction(req_charge, current_timestamp);
                msg!("Auction created for station with ID: {}", report.id);
            }
        }

        Ok(())
    }

    pub fn place_bid(&self, amount: u64, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("Instruction: Place Bid");
        msg!("Amount: {}", amount);
        msg!("Accounts: {:?}", accounts);
        Ok(())
    }
}