use crypto_deposits::*;

pub mod crypto_deposits;
pub mod exchange;
lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_OPTIONS: AssetDepositOptions  = AssetDepositOptions::new();

}
