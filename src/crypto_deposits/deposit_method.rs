use service_sdk::rust_extensions::sorted_vec::EntityWithStrKey;

pub const ETH_ERC20_DEPOSIT_METHOD_ID: &str = "EthErc20";
pub const TRC20_DEPOSIT_METHOD_ID: &str = "Trc20";
pub const BINANCE_BEP20_DEPOSIT_METHOD_ID: &str = "BinanceBep20";

#[derive(Debug, Clone, Copy)]
pub enum CryptoDepositMethod {
    Native,
    EthErc20,
    Trc20,
    BinanceBep20,
}

impl CryptoDepositMethod {
    pub fn from_str(s: &str) -> Self {
        match s {
            ETH_ERC20_DEPOSIT_METHOD_ID => CryptoDepositMethod::EthErc20,
            TRC20_DEPOSIT_METHOD_ID => CryptoDepositMethod::Trc20,
            BINANCE_BEP20_DEPOSIT_METHOD_ID => CryptoDepositMethod::BinanceBep20,
            _ => CryptoDepositMethod::Native,
        }
    }

    pub fn as_str<'s>(&self, asset_id: &'s str) -> &'s str {
        match self {
            CryptoDepositMethod::Native => asset_id,
            CryptoDepositMethod::EthErc20 => ETH_ERC20_DEPOSIT_METHOD_ID,
            CryptoDepositMethod::Trc20 => TRC20_DEPOSIT_METHOD_ID,
            CryptoDepositMethod::BinanceBep20 => BINANCE_BEP20_DEPOSIT_METHOD_ID,
        }
    }

    pub fn is_native(&self) -> bool {
        matches!(self, CryptoDepositMethod::Native)
    }
}

#[derive(Debug, Clone)]
pub struct AssetCryptoDepositMethod {
    pub asset_id: &'static str,
    pub deposit_methods: Vec<CryptoDepositMethod>,
}

impl EntityWithStrKey for AssetCryptoDepositMethod {
    fn get_key(&self) -> &str {
        self.asset_id
    }
}
