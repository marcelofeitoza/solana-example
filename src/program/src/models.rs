use solana_program::msg;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Bid {
    pub bidder: String,
    pub amount: f32,
    pub price_per_amount: f32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Auction {
    pub req_charge: f32,
    pub timestamp: u64,
    pub bids: Vec<Bid>,
    pub ongoing: bool,
    pub winning_bids: Vec<Bid>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Station {
    pub id: String,
    pub latitude: f32,
    pub longitude: f32,
    pub max_capacity: f32,
    pub battery_level: f32,
    pub auction: Option<Auction>,
}

impl Station {
    pub fn finalize_auction(&mut self, timestamp: u64) {
        msg!("Finalizing auction for station with ID: {}", self.id);

        if let Some(auction) = &mut self.auction {
            msg!("Auction ongoing for station with ID: {}", self.id);

            if auction.ongoing && auction.timestamp <= timestamp {
                msg!("Auction ongoing for station with ID: {}", self.id);

                let mut accepted_bids: Vec<Bid> = Vec::new();
                let mut energy_needed = auction.req_charge;

                auction.bids.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());

                for bid in &auction.bids {
                    msg!("Bidder: {}, Amount: {}", bid.bidder, bid.amount);

                    if energy_needed <= 0.0 {
                        msg!("Energy needed is 0.0");
                        break;
                    }

                    if bid.amount <= energy_needed {
                        msg!("Bid amount is less than energy needed");

                        let bid = Bid {
                            bidder: bid.bidder.clone(),
                            amount: bid.amount,
                            price_per_amount: bid.price_per_amount,
                        };
                        accepted_bids.push(bid.clone());
                        energy_needed -= bid.amount;
                    } else {
                        msg!("Bid amount is more than energy needed");

                        let partial_bid = Bid {
                            bidder: bid.bidder.clone(),
                            amount: energy_needed,
                            price_per_amount: bid.price_per_amount,
                        };
                        accepted_bids.push(partial_bid);
                        energy_needed = 0.0;
                    }
                }

                for accepted_bid in &accepted_bids {
                    let accepted_bid = Bid {
                        bidder: accepted_bid.bidder.clone(),
                        amount: accepted_bid.amount,
                        price_per_amount: accepted_bid.price_per_amount,
                    };

                    msg!("Accepted bid: Bidder: {}, Amount: {}", accepted_bid.bidder.clone(), accepted_bid.amount.clone());

                    self.battery_level += accepted_bid.amount;
                    auction.winning_bids.push(accepted_bid);
                }

                msg!("Auction finalized for station with ID: {}", self.id);
                auction.ongoing = false;
            }
        }
    }

    pub fn create_auction(&mut self, req_charge: f32, timestamp: u64) {
        msg!("Creating auction for station with ID: {}", self.id);

        let new_auction = Auction {
            req_charge,
            timestamp,
            bids: Vec::new(),
            ongoing: true,
            winning_bids: Vec::new(),
        };

        msg!("Auction created for station with ID: {}", self.id);
        self.auction = Some(new_auction);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BatteryReportParams {
    pub id: String,
    pub latitude: f32,
    pub longitude: f32,
    pub max_capacity: f32,
    pub battery_level: f32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PlaceBidParams {
    pub id: String,
    pub bidder: String,
    pub amount: f32,
    pub price_per_amount: f32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Action {
    BatteryReport(BatteryReportParams),
    PlaceBid(PlaceBidParams),
}