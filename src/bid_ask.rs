pub trait BidAsk {
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;

    fn get_mid(&self) -> f64 {
        (self.get_ask() + self.get_bid()) * 0.5
    }
}

pub trait BidAskSearch {
    fn get_sell_asset(&self) -> &str;
    fn get_buy_asset(&self) -> &str;
}
