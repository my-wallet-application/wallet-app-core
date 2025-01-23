use rust_extensions::sorted_vec::SortedVecWithStrKey;

use crate::*;

pub struct AssetCryptoDepositWithdrawOptions {
    items: SortedVecWithStrKey<AssetCryptoDepositWithdrawMethod>,
}

impl AssetCryptoDepositWithdrawOptions {
    pub fn new() -> Self {
        let mut items = SortedVecWithStrKey::new();

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: BTC_ID,
            deposit_methods: vec![BTC_ID],
            withdraw_methods: vec![BTC_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: ETH_ID,
            deposit_methods: vec![ETH_ID],
            withdraw_methods: vec![ETH_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: USDT_ID,
            deposit_methods: vec![ETH_ID, TRX_ID, BNB_ID],
            withdraw_methods: vec![ETH_ID, TRX_ID, BNB_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: SOL_ID,
            deposit_methods: vec![SOL_ID],
            withdraw_methods: vec![SOL_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: DOT_ID,
            deposit_methods: vec![DOT_ID],
            withdraw_methods: vec![DOT_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: TRX_ID,
            deposit_methods: vec![TRX_ID],
            withdraw_methods: vec![TRX_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: LTC_ID,
            deposit_methods: vec![LTC_ID],
            withdraw_methods: vec![LTC_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: ADA_ID,
            deposit_methods: vec![ADA_ID],
            withdraw_methods: vec![ADA_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: ICP_ID,
            deposit_methods: vec![ICP_ID],
            withdraw_methods: vec![ICP_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: XRP_ID,
            deposit_methods: vec![XRP_ID],
            withdraw_methods: vec![XRP_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: XLM_ID,
            deposit_methods: vec![XLM_ID],
            withdraw_methods: vec![XLM_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: HBAR_ID,
            deposit_methods: vec![HBAR_ID],
            withdraw_methods: vec![HBAR_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: BNB_ID,
            deposit_methods: vec![BNB_ID],
            withdraw_methods: vec![BNB_ID],
        });

        items.insert_or_replace(AssetCryptoDepositWithdrawMethod {
            asset_id: AVAX_ID,
            deposit_methods: vec![AVAX_ID],
            withdraw_methods: vec![AVAX_ID],
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
