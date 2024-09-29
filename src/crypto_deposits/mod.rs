mod deposit_method;
pub use deposit_method::*;
mod deposit_asset_id;
pub use deposit_asset_id::*;
use rust_extensions::sorted_vec::SortedVecWithStrKey;

pub struct AssetDepositOptions {
    items: SortedVecWithStrKey<AssetCryptoDepositMethod>,
}

impl AssetDepositOptions {
    pub fn new() -> Self {
        let mut items = SortedVecWithStrKey::new();

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: BTC_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: ETH_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: USDT_ID,
            deposit_methods: vec![
                CryptoDepositMethod::EthErc20,
                CryptoDepositMethod::Trc20,
                CryptoDepositMethod::BinanceBep20,
            ],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: SOL_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: DOT_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: TRX_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: LTC_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: ADA_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: ICP_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: XRP_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: XLM_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: HBAR_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: BNB_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositMethod {
            asset_id: AVAX_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        });

        Self { items }
    }

    pub fn find(&self, asset_id: &str) -> Option<&AssetCryptoDepositMethod> {
        self.items.get(asset_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &AssetCryptoDepositMethod> {
        self.items.iter()
    }
}
