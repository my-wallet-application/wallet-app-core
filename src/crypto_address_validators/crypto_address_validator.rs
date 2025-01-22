use crate::*;

pub fn crypto_address_validator(
    address: &str,
    asset_id: &str,
    crypto_method: CryptoMethod,
) -> bool {
    match asset_id {
        ADA_ID => {
            return super::ada_address_validator(address);
        }
        HBAR_ID => {
            return super::hbar_address_validator(address);
        }
        ETH_ID => {
            return super::eth_address_validator(address);
        }
        _ => {}
    }

    true
}
