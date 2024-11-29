use crate::bid_ask::*;

pub struct BidAskMockModel {
    pub bid: f64,
    pub ask: f64,
    pub sell_asset: &'static str,
    pub buy_asset: &'static str,
}

impl BidAsk for BidAskMockModel {
    fn get_bid(&self) -> f64 {
        self.bid
    }

    fn get_ask(&self) -> f64 {
        self.ask
    }
}

impl BidAskAdditionalInfo for BidAskMockModel {
    fn get_sell_asset(&self) -> &str {
        self.sell_asset
    }

    fn get_buy_asset(&self) -> &str {
        self.buy_asset
    }
}
