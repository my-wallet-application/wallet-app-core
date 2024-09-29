use rust_extensions::sorted_vec::EntityWithStrKey;

pub const NATIVE_DEPOSIT_METHOD_ID: &str = "";
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
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            NATIVE_DEPOSIT_METHOD_ID => Some(CryptoDepositMethod::Native),
            ETH_ERC20_DEPOSIT_METHOD_ID => Some(CryptoDepositMethod::EthErc20),
            TRC20_DEPOSIT_METHOD_ID => Some(CryptoDepositMethod::Trc20),
            BINANCE_BEP20_DEPOSIT_METHOD_ID => Some(CryptoDepositMethod::BinanceBep20),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CryptoDepositMethod::Native => NATIVE_DEPOSIT_METHOD_ID,
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
