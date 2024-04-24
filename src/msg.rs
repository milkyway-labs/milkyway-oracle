use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    PostRates {
        denom: String,
        purchase_rate: String,
        redemption_rate: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Config)]
    Config {},

    #[returns(RedemptionRateResponse)]
    RedemptionRate {
        denom: String,
        params: Option<Binary>,
    },

    #[returns(HistoricalRedemptionRatesResponse)]
    HistoricalRedemptionRates {
        denom: String,
        params: Option<Binary>,
        limit: Option<u64>,
    },

    #[returns(PurchaseRateResponse)]
    PurchaseRate {
        denom: String,
        params: Option<Binary>,
    },

    #[returns(HistoricalPurchaseRatesResponse)]
    HistoricalPurchaseRates {
        denom: String,
        params: Option<Binary>,
        limit: Option<u64>,
    },
}

#[cw_serde]
pub struct RedemptionRateResponse {
    pub redemption_rate: Decimal,
    pub update_time: u64,
}

#[cw_serde]
pub struct PurchaseRateResponse {
    pub purchase_rate: Decimal,
    pub update_time: u64,
}

#[cw_serde]
pub struct HistoricalRedemptionRatesResponse {
    pub redemption_rates: Vec<RedemptionRate>,
}

#[cw_serde]
pub struct HistoricalPurchaseRatesResponse {
    pub purchase_rates: Vec<PurchaseRate>,
}

#[cw_serde]
pub struct RedemptionRate {
    pub denom: String,
    pub redemption_rate: Decimal,
    pub update_time: u64,
}

#[cw_serde]
pub struct PurchaseRate {
    pub denom: String,
    pub purchase_rate: Decimal,
    pub update_time: u64,
}

#[cw_serde]
pub struct MigrateMsg {}
