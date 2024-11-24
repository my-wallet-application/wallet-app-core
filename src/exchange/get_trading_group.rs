use std::sync::Arc;

use my_nosql_contracts::trading_groups::TradingGroupMyNoSqlEntity;
use service_sdk::my_no_sql_sdk::reader::MyNoSqlDataReaderTcp;

pub async fn get_trading_trading_group(
    client_id: &str,
    trading_groups: &MyNoSqlDataReaderTcp<TradingGroupMyNoSqlEntity>,
) -> Option<Arc<TradingGroupMyNoSqlEntity>> {
    trading_groups
        .iter_and_find_entity_inside_partition(TradingGroupMyNoSqlEntity::PARTITION_KEY, |itm| {
            itm.default
        })
        .await
}
