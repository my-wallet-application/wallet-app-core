use crate::*;

pub fn validate_crypto_address(address: &str, asset_id: &str, crypto_method: CryptoMethod) -> bool {
    match asset_id {
        ADA_ID => super::ada_address_validator(address),
        HBAR_ID => super::validate_hbar_address(address),
        ETH_ID => super::eth_address_validator(address),
        BTC_ID => super::validate_btc_address(address),
        XRP_ID => super::xrp_address_validator(address),
        BNB_ID => super::eth_address_validator(address),
        TRX_ID => super::trx_address_validator(address),
        DOT_ID => super::dot_address_validator(address),
        ICP_ID => super::icp_address_validator(address),
        SOL_ID => super::sol_address_validator(address),
        XLM_ID => super::xlm_address_validator(address),
        AVAX_ID => super::avax_address_validator(address),
        USDT_ID => match crypto_method {
            CryptoMethod::EthErc20 => super::eth_address_validator(address),
            CryptoMethod::Trc20 => super::trx_address_validator(address),
            CryptoMethod::BinanceBep20 => super::eth_address_validator(address),
            _ => false,
        },
        _ => false,
    }
}
