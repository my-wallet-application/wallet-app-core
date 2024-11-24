use std::{collections::BTreeMap, sync::Arc};

use my_nosql_contracts::trading_groups::{TradingConditionsProfile, TradingGroupMyNoSqlEntity};
use service_sdk::my_no_sql_sdk::reader::MyNoSqlDataReaderTcp;

pub async fn get_trading_conditions(
    trading_groups: &Arc<MyNoSqlDataReaderTcp<TradingGroupMyNoSqlEntity>>,
    trading_condition_profiles: &Arc<MyNoSqlDataReaderTcp<TradingConditionsProfile>>,
    client_id: &str,
) -> Result<BTreeMap<String, Arc<TradingConditionsProfile>>, String> {
    let groups = trading_groups
        .get_by_partition_key_as_vec(TradingGroupMyNoSqlEntity::PARTITION_KEY)
        .await;

    if groups.is_none() {
        return Err("No trading groups found".to_string());
    }

    let groups = groups.unwrap();

    let default_group = groups.into_iter().find(|x| x.default);

    if default_group.is_none() {
        return Err("No default trading group found".to_string());
    }

    let default_group = default_group.unwrap();

    let groups = trading_condition_profiles
        .get_by_partition_key(default_group.get_id())
        .await;

    if groups.is_none() {
        return Err("No trading conditions found".to_string());
    }

    let groups = groups.unwrap();

    Ok(groups)
}
