use crypto_deposit_withdraw::*;

pub mod crypto_deposit_withdraw;
pub mod exchange;
#[cfg(test)]
pub mod test_mocks;

lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_WITHDRAW_OPTIONS: AssetCryptoDepositWithdrawOptions  = AssetCryptoDepositWithdrawOptions::new();
}

pub mod bid_ask;
pub mod crypto_address_validators;
mod crypto_asset_id;
pub use crypto_asset_id::*;
pub mod base58;
pub mod utils;
