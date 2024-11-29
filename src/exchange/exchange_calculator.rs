use crate::bid_ask::*;

#[derive(Debug, Clone, Copy)]
pub enum ExchangeVolume {
    Sell(f64),
    Buy(f64),
}

#[derive(Debug, Clone, Copy)]
pub struct ExchangeCalculation {
    pub sell_volume: f64,
    pub buy_volume: f64,
    pub commission_volume: f64,
}

pub trait CommissionCalculator {
    fn calc_commission(&self, volume: f64) -> f64;
}

pub fn calc_exchange_operation<TBidAsk: BidAsk + BidAskAdditionalInfo>(
    sell_asset: &str,
    buy_asset: &str,
    volume: ExchangeVolume,
    commission_percent: f64,
    bid_ask: &TBidAsk,
) -> ExchangeCalculation {
    match volume {
        ExchangeVolume::Sell(sell_volume) => {
            let commission_volume = sell_volume * commission_percent * 0.01;
            let to_exchange_volume = sell_volume - commission_volume;
            let buy_volume =
                super::utils::calc_buy_amount(sell_asset, buy_asset, to_exchange_volume, bid_ask);

            return ExchangeCalculation {
                sell_volume,
                buy_volume,
                commission_volume,
            };
        }
        ExchangeVolume::Buy(buy_volume) => {
            let sell_volume =
                super::utils::calc_sell_amount(sell_asset, buy_asset, buy_volume, bid_ask);

            let commission_volume = sell_volume * commission_percent * 0.01;

            return ExchangeCalculation {
                sell_volume: sell_volume + commission_volume,
                buy_volume,
                commission_volume,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{exchange, test_mocks::*};

    use super::ExchangeVolume;

    #[test]
    fn test_buy_xrp_for_usd() {
        let bid_ask = BidAskMockModel {
            bid: 0.5,
            ask: 0.6,
            sell_asset: "XRP",
            buy_asset: "USD",
        };

        let result = exchange::exchange_calculator::calc_exchange_operation(
            "XRP",
            "USD",
            ExchangeVolume::Sell(100.0),
            1.0,
            &bid_ask,
        );

        assert_eq!(result.sell_volume, 100.0);
        assert_eq!(result.commission_volume, 1.0);

        assert_eq!(result.buy_volume, 99.0 * bid_ask.bid);

        println!("{:?}", result);
    }

    #[test]
    fn test_sell_usd_buying_xrp() {
        let bid_ask = BidAskMockModel {
            bid: 0.6,
            ask: 0.5,
            sell_asset: "XRP",
            buy_asset: "USD",
        };

        let result = exchange::exchange_calculator::calc_exchange_operation(
            "USD",
            "XRP",
            ExchangeVolume::Buy(100.0),
            1.0,
            &bid_ask,
        );

        assert_eq!(result.sell_volume, 50.5);
        assert_eq!(result.commission_volume, 0.5);
        assert_eq!(result.buy_volume, 100.0);
    }
}
