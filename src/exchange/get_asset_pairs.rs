use std::sync::Arc;

use my_nosql_contracts::AssetPairMyNoSqlEntity;

pub trait GetAssetPairs {
    async fn get_asset_pairs(&self) -> Vec<Arc<AssetPairMyNoSqlEntity>> {
        todo!("Implement me")
    }
}
