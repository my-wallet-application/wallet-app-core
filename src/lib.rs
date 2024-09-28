use crypto_deposits::*;

pub mod crypto_deposits;

lazy_static::lazy_static! {
    pub static ref ASSET_DEPOSIT_METHODS: Vec<AssetCryptoDepositMethod>  = {

        vec![
            AssetCryptoDepositMethod {
             asset_id: BTC_ID,
             deposit_methods: vec![CryptoDepositMethod::Native],
            },

        AssetCryptoDepositMethod {
            asset_id: ETH_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },



        AssetCryptoDepositMethod {
            asset_id: USDT_ID,
            deposit_methods: vec![CryptoDepositMethod::EthErc20, CryptoDepositMethod::Trc20, CryptoDepositMethod::BinanceBep20],
        },


        AssetCryptoDepositMethod {
            asset_id: SOL_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: DOT_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: TRX_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: LTC_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: ADA_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: ICP_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: XRP_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: XLM_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: HBAR_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: BNB_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        },

        AssetCryptoDepositMethod {
            asset_id: AVAX_ID,
            deposit_methods: vec![CryptoDepositMethod::Native],
        }

        ]
    };

}
