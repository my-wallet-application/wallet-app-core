use crate::bid_ask::*;

pub fn calc_buy_amount<TBidAsk: BidAsk + BidAskSearch>(
    sell_asset: &str,
    buy_asset: &str,
    sell_amount: f64,
    bid_ask: &TBidAsk,
) -> f64 {
    if bid_ask.get_sell_asset() == sell_asset && bid_ask.get_buy_asset() == buy_asset {
        return sell_amount * bid_ask.get_bid();
    }

    return sell_amount / bid_ask.get_ask();
}

pub fn calc_width_commission<TBidAsk: BidAsk + BidAskSearch>(
    sell_asset: &str,
    buy_asset: &str,
    sell_amount: f64,
    commission_amount: f64,
    rate: &TBidAsk,
) -> f64 {
    let sell_amount = sell_amount - commission_amount;

    if rate.get_sell_asset() == sell_asset && rate.get_buy_asset() == buy_asset {
        return sell_amount * rate.get_bid();
    }

    return sell_amount / rate.get_ask();
}

pub fn calc_sell_amount<TBidAsk: BidAsk + BidAskSearch>(
    sell_asset: &str,
    buy_asset: &str,
    buy_amount: f64,
    bid_ask: &TBidAsk,
) -> f64 {
    if bid_ask.get_sell_asset() == sell_asset && bid_ask.get_buy_asset() == buy_asset {
        return buy_amount / bid_ask.get_bid();
    }

    return buy_amount * bid_ask.get_ask();
}

pub fn calc_sell_amount_with_commission<TBidAsk: BidAsk + BidAskSearch>(
    sell_asset: &str,
    buy_asset: &str,
    buy_amount: f64,
    commission_amount: f64,
    rate: &TBidAsk,
) -> f64 {
    if rate.get_sell_asset() == sell_asset && rate.get_buy_asset() == buy_asset {
        return buy_amount / rate.get_bid() + commission_amount;
    }

    return buy_amount * rate.get_ask() + commission_amount;
}

#[cfg(test)]
mod tests {
    use super::{BidAsk, BidAskSearch};

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

    impl BidAskSearch for BidAskMockModel {
        fn get_sell_asset(&self) -> &str {
            self.sell_asset
        }

        fn get_buy_asset(&self) -> &str {
            self.buy_asset
        }
    }

    #[test]
    fn test_calc_buy_amount() {
        let bid_ask = BidAskMockModel {
            bid: 10.0,
            ask: 20.0,
            sell_asset: "ETH",
            buy_asset: "USD",
        };

        let buy_amount = super::calc_buy_amount("ETH", "USD", 1.0, &bid_ask);
        assert_eq!(buy_amount, 10.0);

        let buy_amount = super::calc_buy_amount("USD", "ETH", buy_amount, &bid_ask);
        assert_eq!(buy_amount, 0.5);
    }

    #[test]
    fn test_calc_sell_amount() {
        let bid_ask = BidAskMockModel {
            bid: 10.0,
            ask: 20.0,
            sell_asset: "ETH",
            buy_asset: "USD",
        };

        let sell_amount = super::calc_sell_amount("ETH", "USD", 10.0, &bid_ask);
        assert_eq!(sell_amount, 1.0);

        let sell_amount = super::calc_buy_amount("USD", "ETH", 10.0, &bid_ask);
        assert_eq!(sell_amount, 0.5);
    }
}
