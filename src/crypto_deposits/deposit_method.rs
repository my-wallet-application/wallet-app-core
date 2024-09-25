#[derive(Debug, Clone, Copy)]
pub enum DepositMethod {
    Native,
    EthErc20,
    Trc20,
    BinanceBep20,
}

impl DepositMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Native" => Some(DepositMethod::Native),
            "EthErc20" => Some(DepositMethod::EthErc20),
            "Trc20" => Some(DepositMethod::Trc20),
            "BinanceBep20" => Some(DepositMethod::BinanceBep20),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssetDepositMethod {
    pub asset_id: &'static str,
    pub deposit_methods: Vec<DepositMethod>,
}
