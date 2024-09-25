use crypto_deposits::{AssetDepositMethod, DepositMethod};

pub mod crypto_deposits;

lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_METHODS: Vec<AssetDepositMethod>  = {

        vec![
            AssetDepositMethod {
             asset_id: "BTC",
             deposit_methods: vec![DepositMethod::Native],
            },

        AssetDepositMethod {
            asset_id: "ETH",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "ETH",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "USDT",
            deposit_methods: vec![DepositMethod::EthErc20, DepositMethod::Trc20, DepositMethod::BinanceBep20],
        },


        AssetDepositMethod {
            asset_id: "SOL",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "DOT",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "TRX",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "LTC",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "ADA",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "ICP",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "XRP",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "XLM",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "HBAR",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "BNB",
            deposit_methods: vec![DepositMethod::Native],
        },

        AssetDepositMethod {
            asset_id: "AVAX",
            deposit_methods: vec![DepositMethod::Native],
        }

        ]
    };

}

/*
     vec![
        AssetDepositMethod {
        asset_id: "BTC",
        deposit_methods: vec![DepositMethod::Native],
    }]
*/
