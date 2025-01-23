use rust_extensions::sorted_vec::EntityWithStrKey;

#[derive(Debug, Clone)]
pub struct AssetCryptoDepositWithdrawMethod {
    pub asset_id: &'static str,
    pub deposit_methods: Vec<&'static str>,
    pub withdraw_methods: Vec<&'static str>,
}

impl EntityWithStrKey for AssetCryptoDepositWithdrawMethod {
    fn get_key(&self) -> &str {
        self.asset_id
    }
}
