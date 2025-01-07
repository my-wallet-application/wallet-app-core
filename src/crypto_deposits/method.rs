use rust_extensions::sorted_vec::EntityWithStrKey;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum CryptoMethod {
    Native,
    EthErc20,
    Trc20,
    BinanceBep20,
}

impl CryptoMethod {
    pub fn from_str(s: &str) -> Self {
        match s {
            ETH_ERC20_DEPOSIT_METHOD_ID => Self::EthErc20,
            TRC20_DEPOSIT_METHOD_ID => Self::Trc20,
            BINANCE_BEP20_DEPOSIT_METHOD_ID => Self::BinanceBep20,
            _ => Self::Native,
        }
    }

    pub fn as_str<'s>(&self, asset_id: &'s str) -> &'s str {
        match self {
            Self::Native => asset_id,
            Self::EthErc20 => ETH_ERC20_DEPOSIT_METHOD_ID,
            Self::Trc20 => TRC20_DEPOSIT_METHOD_ID,
            Self::BinanceBep20 => BINANCE_BEP20_DEPOSIT_METHOD_ID,
        }
    }

    pub fn is_native(&self) -> bool {
        matches!(self, Self::Native)
    }
}

#[derive(Debug, Clone)]
pub struct AssetCryptoMethod {
    pub asset_id: &'static str,
    pub deposit_methods: Vec<CryptoMethod>,
}

impl EntityWithStrKey for AssetCryptoMethod {
    fn get_key(&self) -> &str {
        self.asset_id
    }
}
