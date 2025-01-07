mod method;
pub use method::*;
mod deposit_asset_id;
pub use deposit_asset_id::*;
mod utils;
pub use utils::*;

pub const ETH_ERC20_DEPOSIT_METHOD_ID: &str = "EthErc20";
pub const TRC20_DEPOSIT_METHOD_ID: &str = "Trc20";
pub const BINANCE_BEP20_DEPOSIT_METHOD_ID: &str = "BinanceBep20";

mod crypto_deposit_withdraw_options;
pub use crypto_deposit_withdraw_options::*;
