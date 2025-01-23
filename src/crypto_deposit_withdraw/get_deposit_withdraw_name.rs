use crate::{BNB_ID, ETH_ID, TRX_ID};

pub fn get_deposit_withdraw_name<'s>(asset_id: &'s str, network_id: &'s str) -> &'s str {
    if asset_id == network_id {
        return asset_id;
    }

    match network_id {
        ETH_ID => "Ethereum (ERC20)",
        TRX_ID => "Tron (TRC20)",
        BNB_ID => "Binance Smart Chain (BEP20)",
        _ => asset_id,
    }
}
