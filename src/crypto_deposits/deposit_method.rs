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
            "" => Some(CryptoDepositMethod::Native),
            "EthErc20" => Some(CryptoDepositMethod::EthErc20),
            "Trc20" => Some(CryptoDepositMethod::Trc20),
            "BinanceBep20" => Some(CryptoDepositMethod::BinanceBep20),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CryptoDepositMethod::Native => "",
            CryptoDepositMethod::EthErc20 => "EthErc20",
            CryptoDepositMethod::Trc20 => "Trc20",
            CryptoDepositMethod::BinanceBep20 => "BinanceBep20",
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
