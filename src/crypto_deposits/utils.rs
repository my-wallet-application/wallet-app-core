use super::*;

pub fn has_address_memo(asset_id: &str) -> bool {
    asset_id == XRP_ID || asset_id == XLM_ID || asset_id == HBAR_ID
}

pub trait BidAsk {
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;

    fn get_mid(&self) -> f64 {
        (self.get_ask() + self.get_bid()) * 0.5
    }
}
