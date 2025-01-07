use rust_extensions::sorted_vec::SortedVecWithStrKey;

use super::*;

pub struct AssetCryptoDepositOptions {
    items: SortedVecWithStrKey<AssetCryptoMethod>,
}

impl AssetCryptoDepositOptions {
    pub fn new() -> Self {
        let mut items = SortedVecWithStrKey::new();

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: BTC_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: ETH_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: USDT_ID,
            deposit_methods: vec![
                CryptoMethod::EthErc20,
                CryptoMethod::Trc20,
                CryptoMethod::BinanceBep20,
            ],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: SOL_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: DOT_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: TRX_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: LTC_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: ADA_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: ICP_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: XRP_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: XLM_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: HBAR_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: BNB_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoMethod {
            asset_id: AVAX_ID,
            deposit_methods: vec![CryptoMethod::Native],
        });

        Self { items }
    }

    pub fn get(&self, asset_id: &str) -> Option<&AssetCryptoMethod> {
        self.items.get(asset_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &AssetCryptoMethod> {
        self.items.iter()
    }
}
