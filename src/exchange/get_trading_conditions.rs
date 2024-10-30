use my_nosql_contracts::trading_groups::{TradingConditionsProfile, TradingGroupMyNoSqlEntity};
use service_sdk::my_no_sql_sdk::reader::MyNoSqlDataReaderTcp;

pub async fn get_trading_conditions(
    trading_conditions: &MyNoSqlDataReaderTcp<TradingConditionsProfile>,
    trading_group: &TradingGroupMyNoSqlEntity,
) {
    todo!("Implement me")
    /*
    let trading_group =
        super::get_trading_trading_group(&client_id, self.get_trading_groups_dict()).await;
         */
}
