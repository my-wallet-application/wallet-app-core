use crypto_deposits::*;

pub mod crypto_deposits;
pub mod exchange;
#[cfg(test)]
pub mod test_mocks;

lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_OPTIONS: AssetCryptoDepositOptions  = AssetCryptoDepositOptions::new();
}

lazy_static::lazy_static! {
    pub static ref ASSET_WITHDRAW_OPTIONS: AssetCryptoWithdrawOptions  = AssetCryptoWithdrawOptions::new();
}
pub mod bid_ask;
