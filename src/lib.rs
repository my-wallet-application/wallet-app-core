use crypto_deposits::*;

pub mod crypto_deposits;
pub mod exchange;
#[cfg(test)]
pub mod test_mocks;

lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_OPTIONS: AssetDepositOptions  = AssetDepositOptions::new();

}
pub mod bid_ask;
