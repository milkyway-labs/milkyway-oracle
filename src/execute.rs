use crate::error::ContractError;
use crate::state::{Rates, CONFIG, MAX_NUM_HISTORICAL_RATES, RATES};
use cosmwasm_std::{ensure, Decimal, DepsMut, Env, MessageInfo, Order, Response};
use std::str::FromStr;

pub fn post_rates(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom: String,
    purchase_rate: String,
    redemption_rate: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    ensure!(
        info.sender == config.admin_address,
        ContractError::Unauthorized {}
    );

    let block_height = env.block.height;
    let update_time = env.block.time.seconds();

    let purchase_rate = Decimal::from_str(&purchase_rate).unwrap();
    let redemption_rate = Decimal::from_str(&redemption_rate).unwrap();

    let rates = Rates {
        purchase_rate,
        redemption_rate,
        update_time,
    };
    RATES.save(deps.storage, (&denom, block_height), &rates)?;

    let prefix = RATES.prefix(&denom);
    // Truncate the rates
    let count = prefix
        .range(deps.storage, None, None, Order::Ascending)
        .count();
    if count > MAX_NUM_HISTORICAL_RATES {
        let first = prefix
            .range(deps.storage, None, None, Order::Ascending)
            .take(1)
            .next()
            .unwrap()?;
        RATES.remove(deps.storage, (&denom, first.0));
    }

    Ok(Response::new()
        .add_attribute("action", "post_rates")
        .add_attribute("denom", denom)
        .add_attribute("purchase_rate", purchase_rate.to_string())
        .add_attribute("redemption_rate", redemption_rate.to_string())
        .add_attribute("update_time", update_time.to_string()))
}
