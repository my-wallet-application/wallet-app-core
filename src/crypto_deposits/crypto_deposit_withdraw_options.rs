use rust_extensions::sorted_vec::SortedVecWithStrKey;

use super::*;

pub struct AssetCryptoDepositWithdrawOptions {
    items: SortedVecWithStrKey<AssetCryptoDepositWithdrawMethod>,
}

impl AssetCryptoDepositWithdrawOptions {
    pub fn new() -> Self {
        let mut items = SortedVecWithStrKey::new();

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: BTC_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: ETH_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: USDT_ID,
            deposit_methods: vec![
                CryptoMethod::EthErc20,
                CryptoMethod::Trc20,
                CryptoMethod::BinanceBep20,
            ],
            withdraw_methods: vec![
                CryptoMethod::EthErc20,
                CryptoMethod::Trc20,
                CryptoMethod::BinanceBep20,
            ],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: SOL_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: DOT_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: TRX_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: LTC_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: ADA_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: ICP_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: XRP_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: XLM_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: HBAR_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: BNB_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: AVAX_ID,
            deposit_methods: vec![CryptoMethod::Native],
            withdraw_methods: vec![CryptoMethod::Native],
        });

        Self { items }
    }

    pub fn get(&self, asset_id: &str) -> Option<&AssetCryptoDepositWithdrawMethod> {
        self.items.get(asset_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &AssetCryptoDepositWithdrawMethod> {
        self.items.iter()
    }
}
