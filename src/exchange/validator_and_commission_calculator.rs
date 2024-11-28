use std::sync::Arc;

use my_nosql_contracts::{trading_groups::*, *};
use service_sdk::{
    async_trait::async_trait, my_logger::LogEventCtx, my_no_sql_sdk::reader::MyNoSqlDataReaderTcp,
};

use crate::bid_ask::*;

#[derive(Debug, Clone)]
pub enum ExchangeValidationError {
    AssetPairNotFound,
    TradingGroupNotFound,
    TradingAssetIsNotConfigured(String),
    TradingConditionNotFound,
    GlobalSettingsNotFound,
    ExchangeBetweenAssetsIsDisabled,
}

pub struct ValidationOkResult {
    pub commission: f64,
    pub commission_wallet_id: String,
    pub asset_pair: Arc<AssetPairMyNoSqlEntity>,
    pub trading_group: Arc<TradingGroupMyNoSqlEntity>,
    pub trading_conditions_profile: Arc<TradingConditionsProfile>,
}

#[async_trait]
pub trait ExchangeValidatorAndCommissionDictsResolver<TBidAsk: BidAsk + Send + Sync + 'static> {
    fn get_trading_groups_dict(&self) -> &MyNoSqlDataReaderTcp<TradingGroupMyNoSqlEntity>;
    fn get_asset_pairs_dict(&self) -> &MyNoSqlDataReaderTcp<AssetPairMyNoSqlEntity>;

    fn get_trading_conditions(&self) -> &MyNoSqlDataReaderTcp<TradingConditionsProfile>;

    fn get_global_settings(&self) -> &MyNoSqlDataReaderTcp<GlobalSettingsMyNoSqlEntity>;

    async fn get_bid_ask(&self, id: &str) -> Option<TBidAsk>;
}

const PROCESS_NAME: &str = "calc_exchange_commission";

pub async fn calc_exchange_commission<TBidAsk: BidAsk + BidAskSearch + Send + Sync + 'static>(
    dicts_resolver: &impl ExchangeValidatorAndCommissionDictsResolver<TBidAsk>,
    client_id: &str,
    sell_asset: &str,
    buy_asset: &str,
    sell_amount: Option<f64>,
    buy_amount: Option<f64>,
) -> Result<ValidationOkResult, ExchangeValidationError> {
    let asset_pair = dicts_resolver
        .get_asset_pairs_dict()
        .iter_and_find_entity_inside_partition(AssetPairMyNoSqlEntity::PARTITION_KEY, |itm| {
            itm.from_asset == sell_asset && itm.to_asset == buy_asset
                || itm.from_asset == buy_asset && itm.to_asset == sell_asset
        })
        .await;

    if asset_pair.is_none() {
        service_sdk::my_logger::LOGGER.write_error(
            PROCESS_NAME,
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
        super::get_trading_trading_group(&client_id, dicts_resolver.get_trading_groups_dict())
            .await;

    if trading_group.is_none() {
        service_sdk::my_logger::LOGGER.write_error(
            PROCESS_NAME,
            "Not trading group found for a client",
            LogEventCtx::new().add("client_id", client_id),
        );

        return Err(ExchangeValidationError::TradingGroupNotFound);
    }

    let trading_group = trading_group.unwrap();

    if trading_group.assets.is_none() {
        service_sdk::my_logger::LOGGER.write_error(
            PROCESS_NAME,
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
            PROCESS_NAME,
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
            PROCESS_NAME,
            "Default trading group does not have BUY asset configured",
            LogEventCtx::new()
                .add("client_id", client_id)
                .add("buy_asset", buy_asset),
        );

        return Err(ExchangeValidationError::TradingAssetIsNotConfigured(
            buy_asset.to_string(),
        ));
    }

    let trading_conditions_profile = dicts_resolver
        .get_trading_conditions()
        .get_entity(trading_group.get_id(), asset_pair.get_id())
        .await;

    if trading_conditions_profile.is_none() {
        service_sdk::my_logger::LOGGER.write_error(
            PROCESS_NAME,
            "No asset pair configured for the trading conditions_profile",
            LogEventCtx::new()
                .add("client_id", client_id)
                .add("buy_asset", buy_asset)
                .add("sell_asset", sell_asset)
                .add("trading_group_id", trading_group.get_id())
                .add("asset_id", asset_pair.get_id()),
        );
        return Err(ExchangeValidationError::TradingConditionNotFound);
    }

    let trading_conditions_profile = trading_conditions_profile.unwrap();

    let global_settings = dicts_resolver
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

    let direct = asset_pair.from_asset == sell_asset;

    let sell_amount = if let Some(sell_amount) = sell_amount {
        sell_amount
    } else {
        let bid_ask = dicts_resolver.get_bid_ask(asset_pair.get_id()).await;

        if bid_ask.is_none() {
            service_sdk::my_logger::LOGGER.write_error(
                PROCESS_NAME,
                "No bid ask found",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("buy_asset", buy_asset)
                    .add("buy_amount", format!("{:?}", buy_amount))
                    .add("sell_amount", format!("{:?}", sell_amount))
                    .add("sell_asset", sell_asset)
                    .add("trading_group_id", trading_group.get_id())
                    .add("asset_id", asset_pair.get_id()),
            );

            return Err(ExchangeValidationError::GlobalSettingsNotFound);
        }

        let bid_ask = bid_ask.unwrap();
        let buy_amount = buy_amount.unwrap();
        super::utils::calc_sell_amount(sell_asset, buy_asset, buy_amount, bid_ask.as_ref())
    };

    let commission = if direct {
        if !trading_conditions_profile.direct_exchange {
            service_sdk::my_logger::LOGGER.write_error(
                PROCESS_NAME,
                "Exchange between assets is disabled",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("buy_asset", buy_asset)
                    .add("sell_asset", sell_asset)
                    .add("trading_group_id", trading_group.get_id())
                    .add("asset_id", asset_pair.get_id()),
            );
            return Err(ExchangeValidationError::ExchangeBetweenAssetsIsDisabled);
        }

        sell_amount * trading_conditions_profile.direct_exchange_commission
    } else {
        if !trading_conditions_profile.reverse_exchange {
            service_sdk::my_logger::LOGGER.write_error(
                PROCESS_NAME,
                "Exchange between assets is disabled",
                LogEventCtx::new()
                    .add("client_id", client_id)
                    .add("buy_asset", buy_asset)
                    .add("sell_asset", sell_asset)
                    .add("trading_group_id", trading_group.get_id())
                    .add("asset_id", asset_pair.get_id()),
            );
            return Err(ExchangeValidationError::ExchangeBetweenAssetsIsDisabled);
        }
        sell_amount * trading_conditions_profile.direct_exchange_commission
    };

    return Ok(ValidationOkResult {
        commission: commission * 0.01,
        commission_wallet_id: global_settings.corporate_account_id.to_string(),
        asset_pair,
        trading_conditions_profile,
        trading_group,
    });
}
