use std::sync::Arc;

use my_nosql_contracts::{trading_groups::*, *};
use service_sdk::{
    async_trait::async_trait, my_logger::LogEventCtx, my_no_sql_sdk::reader::MyNoSqlDataReaderTcp,
};

#[derive(Debug, Clone)]
pub enum ExchangeValidationError {
    AssetPairNotFound,
    TradingGroupNotFound,
    TradingAssetIsNotConfigured(String),
    TradingConditionNotFound,
    GlobalSettingsNotFound,
}

pub struct ValidationOkResult {
    pub commission: f64,
    pub commission_wallet_id: String,
    pub asset_pair: Arc<AssetPairMyNoSqlEntity>,
    pub trading_group: Arc<TradingGroupMyNoSqlEntity>,
    pub trading_condition: Arc<TradingConditionsProfile>,
}

#[async_trait]
pub trait ExchangeValidatorAndCommissionCalculator {
    fn get_trading_groups_dict(&self) -> &MyNoSqlDataReaderTcp<TradingGroupMyNoSqlEntity>;
    fn get_asset_pairs_dict(&self) -> &MyNoSqlDataReaderTcp<AssetPairMyNoSqlEntity>;

    fn get_trading_conditions(&self) -> &MyNoSqlDataReaderTcp<TradingConditionsProfile>;

    fn get_global_settings(&self) -> &MyNoSqlDataReaderTcp<GlobalSettingsMyNoSqlEntity>;

    async fn calc_commission(
        &self,
        client_id: &str,
        sell_asset: &str,
        buy_asset: &str,
        sell_amount: f64,
    ) -> Result<ValidationOkResult, ExchangeValidationError> {
        let asset_pair = self
            .get_asset_pairs_dict()
            .iter_and_find_entity_inside_partition(AssetPairMyNoSqlEntity::PARTITION_KEY, |itm| {
                itm.from_asset == sell_asset && itm.to_asset == buy_asset
                    || itm.from_asset == buy_asset && itm.to_asset == sell_asset
            })
            .await;

        if asset_pair.is_none() {
            service_sdk::my_logger::LOGGER.write_error(
                "get swap commission",
                "Invalid assets to execute swap operation",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("sell_asset", sell_asset)
                    .add("buy_asset", buy_asset),
            );

            return Err(ExchangeValidationError::AssetPairNotFound);
        }

        let asset_pair = asset_pair.unwrap();

        let trading_group =
            super::get_trading_trading_group(&client_id, self.get_trading_groups_dict()).await;

        if trading_group.is_none() {
            service_sdk::my_logger::LOGGER.write_error(
                "get swap commission",
                "Not trading group found for a client",
                LogEventCtx::new().add("client_id", client_id),
            );

            return Err(ExchangeValidationError::TradingGroupNotFound);
        }

        let trading_group = trading_group.unwrap();

        if trading_group.assets.is_none() {
            service_sdk::my_logger::LOGGER.write_error(
                "get swap commission",
                "Trading group does not have assets configured",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("trading_group_id", trading_group.get_id()),
            );

            return Err(ExchangeValidationError::TradingGroupNotFound);
        }

        let mut has_sell_asset = false;
        let mut has_buy_asset = false;
        for asset_id in trading_group.assets.as_ref().unwrap() {
            if asset_id == sell_asset {
                has_sell_asset = true;
            }

            if asset_id == buy_asset {
                has_buy_asset = true;
            }

            if has_sell_asset && has_buy_asset {
                break;
            }
        }

        if !has_sell_asset {
            service_sdk::my_logger::LOGGER.write_error(
                "get swap commission",
                "Default trading group does not have SELL asset configured",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("sell_asset", sell_asset),
            );
            return Err(ExchangeValidationError::TradingAssetIsNotConfigured(
                sell_asset.to_string(),
            ));
        }

        if !has_buy_asset {
            service_sdk::my_logger::LOGGER.write_error(
                "get swap commission",
                "Default trading group does not have BUY asset configured",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("buy_asset", buy_asset),
            );

            return Err(ExchangeValidationError::TradingAssetIsNotConfigured(
                buy_asset.to_string(),
            ));
        }

        /*

        let trading_group = self
            .get_trading_groups_dict()
            .iter_and_find_entity_inside_partition(
                TradingGroupMyNoSqlEntity::PARTITION_KEY,
                |itm| {
                    if !itm.default {
                        return false;
                    }

                    let mut has_sell_asset = false;
                    let mut has_buy_asset = false;

                    if let Some(assets) = itm.assets.as_ref() {
                        for asset_id in assets {
                            if asset_id == sell_asset {
                                has_sell_asset = true;
                            }

                            if asset_id == buy_asset {
                                has_buy_asset = true;
                            }

                            if has_sell_asset && has_buy_asset {
                                break;
                            }
                        }
                    }

                    if !has_sell_asset {
                        service_sdk::my_logger::LOGGER.write_error(
                            "get swap commission",
                            "Default trading group does not have SELL asset configured",
                            LogEventCtx::new()
                                .add("client_id", client_id)
                                .add("sell_asset", sell_asset),
                        );
                    }

                    if !has_buy_asset {
                        service_sdk::my_logger::LOGGER.write_error(
                            "get swap commission",
                            "Default trading group does not have BUY asset configured",
                            LogEventCtx::new()
                                .add("client_id", client_id)
                                .add("buy_asset", buy_asset),
                        );
                    }

                    has_buy_asset && has_sell_asset
                },
            )
            .await;

        if trading_group.is_none() {}

        let trading_group = trading_group.unwrap();

        let trading_condition = self
            .get_trading_conditions()
            .get_entity(
                &trading_group.trading_conditions_profile_id,
                asset_pair.get_id(),
            )
            .await;

        if trading_condition.is_none() {
            service_sdk::my_logger::LOGGER.write_error(
                "get swap commission",
                "No trading condition found",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("trading_group_id", trading_group.get_id())
                    .add(
                        "trading_condition_id",
                        trading_group.trading_conditions_profile_id.to_string(),
                    ),
            );

            return Err(ExchangeValidationError::TradingConditionNotFound);
        }

        let trading_condition = trading_condition.unwrap();

        let global_settings = self
            .get_global_settings()
            .get_entity(
                GlobalSettingsMyNoSqlEntity::PARTITION_KEY,
                GlobalSettingsMyNoSqlEntity::ROW_KEY,
            )
            .await;

        if global_settings.is_none() {
            service_sdk::my_logger::LOGGER.write_error(
                "get swap commission",
                "No global settings found",
                LogEventCtx::new().add("client_id", client_id),
            );

            return Err(ExchangeValidationError::GlobalSettingsNotFound);
        }

        let global_settings = global_settings.unwrap();

        let commission = if asset_pair.from_asset == sell_asset {
            sell_amount * trading_condition.direct_exchange_commission
        } else {
            sell_amount * trading_condition.reverse_exchange_commission
        };

        return Ok(ValidationOkResult {
            commission,
            commission_wallet_id: global_settings.corporate_account_id.to_string(),
            asset_pair,
            trading_condition,
            trading_group,
        });


        */

        todo!("Implement the rest of the logic")
    }
}
