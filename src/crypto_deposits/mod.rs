pub const ETH_ERC20_DEPOSIT_METHOD_ID: &str = "EthErc20";
pub const TRC20_DEPOSIT_METHOD_ID: &str = "Trc20";
pub const BINANCE_BEP20_DEPOSIT_METHOD_ID: &str = "BinanceBep20";

mod crypto_deposit_withdraw_options;
pub use crypto_deposit_withdraw_options::*;
