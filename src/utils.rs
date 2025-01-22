use crate::*;

pub fn has_address_memo(asset_id: &str) -> bool {
    asset_id == XRP_ID || asset_id == XLM_ID || asset_id == HBAR_ID
}
