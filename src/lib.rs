use crypto_deposits::{AssetCryptoDepositMethod, CryptoDepositMethod};

pub mod crypto_deposits;

lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_METHODS: Vec<AssetCryptoDepositMethod>  = {

        vec![
            AssetCryptoDepositMethod {
             asset_id: "BTC",
             deposit_methods: vec![CryptoDepositMethod::Native],
            },

        AssetCryptoDepositMethod {
            asset_id: "ETH",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "ETH",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "USDT",
            deposit_methods: vec![CryptoDepositMethod::EthErc20, CryptoDepositMethod::Trc20, CryptoDepositMethod::BinanceBep20],
        },


        AssetCryptoDepositMethod {
            asset_id: "SOL",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "DOT",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "TRX",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "LTC",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "ADA",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "ICP",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "XRP",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "XLM",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "HBAR",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "BNB",
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: "AVAX",
            deposit_methods: vec![CryptoDepositMethod::Native],
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
