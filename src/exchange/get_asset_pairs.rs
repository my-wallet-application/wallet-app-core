use std::sync::Arc;

use my_nosql_contracts::{
    trading_groups::{TradingConditionsProfile, TradingGroupMyNoSqlEntity},
    AssetMyNoSqlEntity, AssetPairMyNoSqlEntity,
};
use service_sdk::my_no_sql_sdk::reader::MyNoSqlDataReaderTcp;

pub trait GetAssetsPairsDictsResolver {
    fn get_assets_pairs_dict(&self) -> &Arc<MyNoSqlDataReaderTcp<AssetPairMyNoSqlEntity>>;
    fn get_assets_dict(&self) -> &Arc<MyNoSqlDataReaderTcp<AssetMyNoSqlEntity>>;
    fn get_trading_groups_dict(&self) -> &Arc<MyNoSqlDataReaderTcp<TradingGroupMyNoSqlEntity>>;
    fn get_trading_conditions_dict(&self) -> &Arc<MyNoSqlDataReaderTcp<TradingConditionsProfile>>;
}

pub async fn get_assets_pairs<TResult>(
    dicts_resolver: &impl GetAssetsPairsDictsResolver,
    client_id: &str,
    convert: impl Fn(&AssetPairMyNoSqlEntity, &TradingConditionsProfile) -> TResult,
) -> Result<Vec<TResult>, String> {
    let asset_pairs = dicts_resolver
        .get_assets_pairs_dict()
        .get_by_partition_key_as_vec(AssetPairMyNoSqlEntity::PARTITION_KEY)
        .await
        .unwrap_or_default();

    let trading_conditions = super::get_trading_conditions(
        &dicts_resolver.get_trading_groups_dict(),
        &dicts_resolver.get_trading_conditions_dict(),
        client_id,
    )
    .await?;

    let mut result = Vec::with_capacity(asset_pairs.len());

    let assets_dict = dicts_resolver.get_assets_dict();

    for asset_pair in asset_pairs {
        if !asset_pair.is_enabled {
            continue;
        }

        let asset_by_trading_condition = trading_conditions.get(asset_pair.get_id());

        if asset_by_trading_condition.is_none() {
            continue;
        }

        let asset_by_trading_condition = asset_by_trading_condition.unwrap();

        if let Some(asset) = assets_dict
            .get_entity(AssetMyNoSqlEntity::PARTITION_KEY, &asset_pair.from_asset)
            .await
        {
            if asset.is_enabled == false {
                continue;
            }
        } else {
            continue;
        }

        if let Some(asset) = assets_dict
            .get_entity(AssetMyNoSqlEntity::PARTITION_KEY, &asset_pair.to_asset)
            .await
        {
            if asset.is_enabled == false {
                continue;
            }
        } else {
            continue;
        }

        result.push(convert(&asset_pair, asset_by_trading_condition));
    }

    Ok(result)
}
