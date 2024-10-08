pub trait BidAsk {
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;
    fn sell_asset(&self) -> &str;
    fn buy_asset(&self) -> &str;
}
