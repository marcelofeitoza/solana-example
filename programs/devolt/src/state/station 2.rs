use borsh::{BorshDeserialize, BorshSerialize};

use anchor_lang::prelude::*;

#[derive(Debug)]
#[account]
pub struct Station {
    pub auth: Pubkey,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub max_capacity: f64,
    pub battery_level: f64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Bid {
    pub bidder: String,
    pub amount: f64,
    pub price_per_amount: f64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Auction {
    pub req_charge: f64,
    pub timestamp: u64,
    pub bids: Vec<Bid>,
    pub ongoing: bool,
    pub winning_bids: Vec<Bid>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateBateryReportArgs {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub max_capacity: f64,
    pub battery_level: f64,
}

impl Station {
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED: &[u8] = b"station";

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>();
}
