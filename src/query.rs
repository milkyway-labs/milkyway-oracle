use crate::msg::{
    HistoricalPurchaseRatesResponse, HistoricalRedemptionRatesResponse, PurchaseRate,
    PurchaseRateResponse, RedemptionRate, RedemptionRateResponse,
};
use crate::state::{Rates, RATES};
use cosmwasm_std::{Binary, Deps, Order, StdError, StdResult};

pub fn query_purchase_rate(
    deps: Deps,
    denom: String,
    params: Option<Binary>,
) -> StdResult<PurchaseRateResponse> {
    if params.is_some() {
        return Err(StdError::generic_err(
            "invalid query request - params must be None",
        ));
    }

    match get_latest_rates(deps, denom)? {
        Some(rates) => Ok(PurchaseRateResponse {
            purchase_rate: rates.purchase_rate,
            update_time: rates.update_time,
        }),
        None => Err(StdError::generic_err("purchase rate not found")),
    }
}

pub fn query_historical_purchase_rates(
    deps: Deps,
    denom: String,
    params: Option<Binary>,
    limit: Option<u64>,
) -> StdResult<HistoricalPurchaseRatesResponse> {
    if params.is_some() {
        return Err(StdError::generic_err(
            "invalid query request - params must be None",
        ));
    }

    let historical_rates = get_historical_rates(deps, denom.clone(), limit)?;

    Ok(HistoricalPurchaseRatesResponse {
        purchase_rates: historical_rates
            .iter()
            .map(|v| PurchaseRate {
                denom: denom.clone(),
                purchase_rate: v.purchase_rate,
                update_time: v.update_time,
            })
            .collect(),
    })
}

pub fn query_redemption_rate(
    deps: Deps,
    denom: String,
    params: Option<Binary>,
) -> StdResult<RedemptionRateResponse> {
    if params.is_some() {
        return Err(StdError::generic_err(
            "invalid query request - params must be None",
        ));
    }

    match get_latest_rates(deps, denom)? {
        Some(rates) => Ok(RedemptionRateResponse {
            redemption_rate: rates.redemption_rate,
            update_time: rates.update_time,
        }),
        None => Err(StdError::generic_err("redemption rate not found")),
    }
}

pub fn query_historical_redemption_rates(
    deps: Deps,
    denom: String,
    params: Option<Binary>,
    limit: Option<u64>,
) -> StdResult<HistoricalRedemptionRatesResponse> {
    if params.is_some() {
        return Err(StdError::generic_err(
            "invalid query request - params must be None",
        ));
    }

    let historical_rates = get_historical_rates(deps, denom.clone(), limit)?;

    Ok(HistoricalRedemptionRatesResponse {
        redemption_rates: historical_rates
            .iter()
            .map(|v| RedemptionRate {
                denom: denom.clone(),
                redemption_rate: v.redemption_rate,
                update_time: v.update_time,
            })
            .collect(),
    })
}

pub fn get_latest_rates(deps: Deps, denom: String) -> StdResult<Option<Rates>> {
    RATES
        .prefix(&denom)
        .range(deps.storage, None, None, Order::Descending)
        .take(1)
        .next()
        .transpose()
        .map(|v| v.map(|(_, rates)| rates))
}

pub fn get_historical_rates(
    deps: Deps,
    denom: String,
    limit: Option<u64>,
) -> StdResult<Vec<Rates>> {
    // If limit is not specified, use usize::MAX to get all rates
    let limit: usize = limit.unwrap_or(u64::MAX).try_into().unwrap_or(usize::MAX);
    RATES
        .prefix(&denom)
        .range(deps.storage, None, None, Order::Descending)
        .take(limit)
        .map(|v| v.map(|(_, rates)| rates))
        .collect()
}
