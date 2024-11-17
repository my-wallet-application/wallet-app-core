use std::sync::Arc;

use my_nosql_contracts::AssetPairMyNoSqlEntity;
use service_sdk::async_trait::async_trait;

#[async_trait]
pub trait GetAssetPairs {
    async fn get_asset_pairs(&self) -> Vec<Arc<AssetPairMyNoSqlEntity>>;
}
