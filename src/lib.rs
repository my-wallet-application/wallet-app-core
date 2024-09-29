use crypto_deposits::*;

pub mod crypto_deposits;

lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_METHODS: AssetDepositMethods  = AssetDepositMethods::new();

}
