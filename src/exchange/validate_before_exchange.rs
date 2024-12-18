use std::sync::Arc;

use my_nosql_contracts::{trading_groups::*, *};
use service_sdk::{my_logger::LogEventCtx, my_no_sql_sdk::reader::MyNoSqlDataReaderTcp};

#[derive(Debug, Clone)]
pub enum ExchangeValidationError {
    AssetPairNotFound,
    TradingGroupNotFound,
    TradingAssetIsNotConfigured(String),
    TradingConditionNotFound,
    GlobalSettingsNotFound,
    ExchangeBetweenAssetsIsDisabled,
}

#[derive(Debug, Clone)]
pub struct ExchangeQuoteValidationResult {
    pub asset_pair: Arc<AssetPairMyNoSqlEntity>,
    pub trading_group: Arc<TradingGroupMyNoSqlEntity>,
    pub trading_conditions_profile: Arc<TradingConditionsProfile>,
    pub commission_client_id: String,
    pub commission_percent: f64,
}

pub trait ExchangeValidationDependenciesResolver {
    fn get_trading_groups_dict(&self) -> &MyNoSqlDataReaderTcp<TradingGroupMyNoSqlEntity>;
    fn get_asset_pairs_dict(&self) -> &MyNoSqlDataReaderTcp<AssetPairMyNoSqlEntity>;

    fn get_trading_conditions(&self) -> &MyNoSqlDataReaderTcp<TradingConditionsProfile>;

    fn get_global_settings(&self) -> &MyNoSqlDataReaderTcp<GlobalSettingsMyNoSqlEntity>;
}

const PROCESS_NAME: &str = "validate_before_exchange";

pub async fn validate_before_exchange(
    dependency_resolver: &impl ExchangeValidationDependenciesResolver,
    client_id: &str,
    sell_asset: &str,
    buy_asset: &str,
) -> Result<ExchangeQuoteValidationResult, ExchangeValidationError> {
    let asset_pair = dependency_resolver
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
        super::get_trading_trading_group(&client_id, dependency_resolver.get_trading_groups_dict())
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

    let trading_conditions_profile = dependency_resolver
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

    let global_settings = dependency_resolver
        .get_global_settings()
        .get_entity(
            GlobalSettingsMyNoSqlEntity::PARTITION_KEY,
            GlobalSettingsMyNoSqlEntity::ROW_KEY,
        )
        .await;

    if global_settings.is_none() {
        service_sdk::my_logger::LOGGER.write_error(
            PROCESS_NAME,
            "No global settings found",
            LogEventCtx::new().add("client_id", client_id),
        );

        return Err(ExchangeValidationError::GlobalSettingsNotFound);
    }

    let global_settings = global_settings.unwrap();

    let direct = asset_pair.from_asset == sell_asset;

    let commission_percent = if direct {
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
        trading_conditions_profile.direct_exchange_commission
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
        trading_conditions_profile.reverse_exchange_commission
    };

    return Ok(ExchangeQuoteValidationResult {
        commission_client_id: global_settings.corporate_account_id.to_string(),
        asset_pair,
        trading_conditions_profile,
        trading_group,
        commission_percent,
    });
}

/*
pub async fn calc_exchange_commission<TBidAsk: BidAsk + BidAskSearch + Send + Sync + 'static>(
    dependency_resolver: &impl ExchangeCommissionCalculatorDependenciesResolver<TBidAsk>,
    client_id: &str,
    sell_asset: &str,
    buy_asset: &str,
    amount: ConvertAmount,
) -> Result<ExchangeCalculatorResult<TBidAsk>, ExchangeValidationError> {
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

    let trading_conditions_profile = dependency_resolver
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

    let global_settings = dependency_resolver
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

    let bid_ask = dependency_resolver
        .get_bid_ask_by_id(asset_pair.get_id())
        .await;

    if bid_ask.is_none() {
        service_sdk::my_logger::LOGGER.write_error(
            PROCESS_NAME,
            "No bid ask found",
            LogEventCtx::new()
                .add("client_id", client_id)
                .add("buy_asset", buy_asset)
                .add("amount", format!("{:?}", amount))
                .add("sell_asset", sell_asset)
                .add("trading_group_id", trading_group.get_id())
                .add("asset_id", asset_pair.get_id()),
        );

        return Err(ExchangeValidationError::GlobalSettingsNotFound);
    }

    let bid_ask = bid_ask.unwrap();

    let sell_wallet_balance = dependency_resolver
        .get_wallet_balance(client_id, sell_asset)
        .await;

    let sell_amount = match amount {
        ConvertAmount::SellAmount(sell_amount) => {
            if sell_amount > sell_wallet_balance {
                return Err(ExchangeValidationError::NotEnoughFunds);
            }
            sell_amount
        }
        ConvertAmount::BuyAmount(buy_amount) => {
            let sell_amount =
                super::utils::calc_sell_amount(sell_asset, buy_asset, buy_amount, &bid_ask);

            if sell_amount > sell_wallet_balance {
                return Err(ExchangeValidationError::NotEnoughFunds);
            }

            sell_amount
        }
        ConvertAmount::SellMax => sell_wallet_balance,
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

    let commission = commission * 0.01;

    let sell_amount = sell_amount - commission;

    let buy_amount = super::utils::calc_buy_amount(sell_asset, buy_asset, sell_amount, &bid_ask);

    return Ok(ExchangeCalculatorResult {
        commission,
        commission_wallet_id: global_settings.corporate_account_id.to_string(),
        asset_pair,
        trading_conditions_profile,
        trading_group,
        sell_amount,
        buy_amount,
        bid_ask,
    });
}
 */
