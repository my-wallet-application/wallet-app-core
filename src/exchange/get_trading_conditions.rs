use std::{collections::BTreeMap, sync::Arc};

use my_nosql_contracts::trading_groups::{TradingConditionsProfile, TradingGroupMyNoSqlEntity};
use service_sdk::my_no_sql_sdk::reader::MyNoSqlDataReaderTcp;

pub trait GetTradingConditionsDictsResolver {
    fn get_trading_groups(&self) -> &Arc<MyNoSqlDataReaderTcp<TradingGroupMyNoSqlEntity>>;
    fn get_trading_condition_profiles(
        &self,
    ) -> &Arc<MyNoSqlDataReaderTcp<TradingConditionsProfile>>;
}

pub async fn get_trading_conditions(
    dicts_resolver: &impl GetTradingConditionsDictsResolver,
    client_id: &str,
) -> Result<BTreeMap<String, Arc<TradingConditionsProfile>>, String> {
    let groups = dicts_resolver
        .get_trading_groups()
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

    let groups = dicts_resolver
        .get_trading_condition_profiles()
        .get_by_partition_key(default_group.get_id())
        .await;

    if groups.is_none() {
        return Err("No trading conditions found".to_string());
    }

    let groups = groups.unwrap();

    Ok(groups)
}
