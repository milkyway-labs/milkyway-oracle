#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use semver::Version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::{execute, query};

const CONTRACT_NAME: &str = "crates.io:milkyway-oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        admin_address: deps.api.addr_validate(&msg.admin_address)?,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin_address", msg.admin_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PostRates {
            denom,
            purchase_rate,
            redemption_rate,
        } => execute::post_rates(deps, env, info, denom, purchase_rate, redemption_rate),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&CONFIG.load(deps.storage)?),
        QueryMsg::RedemptionRate { denom, params } => {
            to_json_binary(&query::query_redemption_rate(deps, denom, params)?)
        }
        QueryMsg::HistoricalRedemptionRates {
            denom,
            params,
            limit,
        } => to_json_binary(&query::query_historical_redemption_rates(
            deps, denom, params, limit,
        )?),
        QueryMsg::PurchaseRate { denom, params } => {
            to_json_binary(&query::query_purchase_rate(deps, denom, params)?)
        }
        QueryMsg::HistoricalPurchaseRates {
            denom,
            params,
            limit,
        } => to_json_binary(&query::query_historical_purchase_rates(
            deps, denom, params, limit,
        )?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let current_version = cw2::get_contract_version(deps.storage)?;
    if current_version.contract != CONTRACT_NAME {
        return Err(ContractError::InvalidContract {});
    }

    let version: Version = current_version
        .version
        .parse()
        .map_err(|_| ContractError::InvalidContractVersion {})?;
    let new_version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|_| ContractError::InvalidContractVersion {})?;

    // Cannot migrate with older version
    if version > new_version {
        return Err(ContractError::InvalidContractVersion {});
    }
    // Cannot migrate with the same version
    if version == new_version {
        return Err(ContractError::InvalidContractVersion {});
    }

    // migrate data
    // none

    // set new contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::contract::{execute, instantiate, query, migrate};
    use crate::msg::{
        MigrateMsg, ExecuteMsg, HistoricalPurchaseRatesResponse, HistoricalRedemptionRatesResponse,
        InstantiateMsg, PurchaseRate, PurchaseRateResponse, 
        QueryMsg, RedemptionRateResponse, RedemptionRate,
    };
    use crate::state::Config;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{attr, from_json, to_json_binary, Decimal, Empty, Env, MessageInfo, OwnedDeps};

    const ADMIN_ADDRESS: &str = "my_address";

    fn default_mock() -> (
        OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        Env,
        MessageInfo,
    ) {
        let deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADMIN_ADDRESS, &[]);

        (deps, env, info)
    }

    // instantiate with the admin address
    fn default_instantiate() -> (
        OwnedDeps<MockStorage, MockApi, MockQuerier, Empty>,
        Env,
        MessageInfo,
    ) {
        let (mut deps, env, info) = default_mock();

        let msg = InstantiateMsg {
            admin_address: ADMIN_ADDRESS.to_string(),
        };

        let resp = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            resp.attributes,
            vec![
                attr("action", "instantiate"),
                attr("admin_address", ADMIN_ADDRESS.to_string()),
            ]
        );
        (deps, env, info)
    }

    #[test]
    fn test_config() {
        let (deps, env, _info) = default_instantiate();
        let msg = QueryMsg::Config {};
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let resp:Config = from_json(&resp).unwrap();
        assert_eq!(
            resp.admin_address,
            ADMIN_ADDRESS.to_string()
        );
    }

    #[test]
    fn test_rates_not_found() {
        let (deps, env, _info) = default_instantiate();
        let denom = "factory/denom";

        let msg = QueryMsg::PurchaseRate {
            denom: denom.to_string(),
            params: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg);
        assert_eq!(resp.unwrap_err().to_string(), "Generic error: purchase rate not found");

        let msg = QueryMsg::RedemptionRate {
            denom: denom.to_string(),
            params: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg);
        assert_eq!(resp.unwrap_err().to_string(), "Generic error: redemption rate not found");
    }

    #[test]
    fn test_params_not_none() {
        let (deps, env, _info) = default_instantiate();
        let denom = "factory/denom";

        let msg = QueryMsg::PurchaseRate {
            denom: denom.to_string(),
            params: Some(to_json_binary("test").unwrap()),
        };
        let resp = query(deps.as_ref(), env.clone(), msg);
        assert_eq!(resp.unwrap_err().to_string(), "Generic error: invalid query request - params must be None");

        let msg = QueryMsg::RedemptionRate {
            denom: denom.to_string(),
            params: Some(to_json_binary("test").unwrap()),
        };
        let resp = query(deps.as_ref(), env.clone(), msg);
        assert_eq!(resp.unwrap_err().to_string(), "Generic error: invalid query request - params must be None");
       
        let msg = QueryMsg::HistoricalPurchaseRates {
            denom: denom.to_string(),
            params: Some(to_json_binary("test").unwrap()),
            limit: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg);
        assert_eq!(resp.unwrap_err().to_string(), "Generic error: invalid query request - params must be None");
   
        let msg = QueryMsg::HistoricalRedemptionRates {
            denom: denom.to_string(),
            params: Some(to_json_binary("test").unwrap()),
            limit: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg);
        assert_eq!(resp.unwrap_err().to_string(), "Generic error: invalid query request - params must be None");
    }

    #[test]
    fn test_post_rates() {
        // Instantiate contract
        let (mut deps, env, info) = default_instantiate();
        let denom = "factory/denom";

        // Post rates
        let msg = ExecuteMsg::PostRates {
            denom: denom.to_string(),
            purchase_rate: "0.9".to_string(),
            redemption_rate: "1.1".to_string(),
        };
        let resp = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(
            resp.attributes,
            vec![
                attr("action", "post_rates"),
                attr("denom", denom),
                attr("purchase_rate", "0.9"),
                attr("redemption_rate", "1.1"),
                attr("update_time", "1571797419"),
            ]
        );

        let msg = QueryMsg::PurchaseRate {
            denom: denom.to_string(),
            params: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let resp: PurchaseRateResponse = from_json(&resp).unwrap();
        assert_eq!(
            resp,
            PurchaseRateResponse {
                purchase_rate: Decimal::from_str("0.9").unwrap(),
                update_time: 1571797419,
            }
        );

        let msg = QueryMsg::RedemptionRate {
            denom: denom.to_string(),
            params: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let resp: RedemptionRateResponse = from_json(&resp).unwrap();
        assert_eq!(
            resp,
            RedemptionRateResponse {
                redemption_rate: Decimal::from_str("1.1").unwrap(),
                update_time: 1571797419,
            }
        );
    }

    #[test]
    fn test_historical_post_rates() {
        // Instantiate contract
        let (mut deps, mut env, info) = default_instantiate();
        let denom = "factory/denom";

        // Post rates
        let msg = ExecuteMsg::PostRates {
            denom: denom.to_string(),
            purchase_rate: "0.9".to_string(),
            redemption_rate: "1.1".to_string(),
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        env.block.height += 10;
        env.block.time = env.block.time.plus_seconds(50);

        // Post rates again
        let msg = ExecuteMsg::PostRates {
            denom: denom.to_string(),
            purchase_rate: "0.8".to_string(),
            redemption_rate: "1.2".to_string(),
        };
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let msg = QueryMsg::PurchaseRate {
            denom: denom.to_string(),
            params: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let resp: PurchaseRateResponse = from_json(&resp).unwrap();
        assert_eq!(
            resp,
            PurchaseRateResponse {
                purchase_rate: Decimal::from_str("0.8").unwrap(),
                update_time: 1571797469,
            }
        );

        let msg = QueryMsg::HistoricalPurchaseRates {
            denom: denom.to_string(),
            params: None,
            limit: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let resp: HistoricalPurchaseRatesResponse = from_json(&resp).unwrap();
        assert_eq!(
            resp,
            HistoricalPurchaseRatesResponse {
                purchase_rates: vec![
                    PurchaseRate {
                        denom: denom.to_string(),
                        purchase_rate: Decimal::from_str("0.8").unwrap(),
                        update_time: 1571797469,
                    },
                    PurchaseRate {
                        denom: denom.to_string(),
                        purchase_rate: Decimal::from_str("0.9").unwrap(),
                        update_time: 1571797419,
                    },
                ]
            }
        );

        let msg = QueryMsg::RedemptionRate {
            denom: denom.to_string(),
            params: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let resp: RedemptionRateResponse = from_json(&resp).unwrap();
        assert_eq!(
            resp,
            RedemptionRateResponse {
                redemption_rate: Decimal::from_str("1.2").unwrap(),
                update_time: 1571797469,
            }
        );

        let msg = QueryMsg::HistoricalRedemptionRates {
            denom: denom.to_string(),
            params: None,
            limit: None,
        };
        let resp = query(deps.as_ref(), env.clone(), msg).unwrap();
        let resp: HistoricalRedemptionRatesResponse = from_json(&resp).unwrap();
        assert_eq!(
            resp,
            HistoricalRedemptionRatesResponse {
                redemption_rates: vec![
                    RedemptionRate {
                        denom: denom.to_string(),
                        redemption_rate: Decimal::from_str("1.2").unwrap(),
                        update_time: 1571797469,
                    },
                    RedemptionRate {
                        denom: denom.to_string(),
                        redemption_rate: Decimal::from_str("1.1").unwrap(),
                        update_time: 1571797419,
                    },
                ]
            }
        );
    }
}
