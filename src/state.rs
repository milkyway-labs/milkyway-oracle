use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");

pub const MAX_NUM_HISTORICAL_RATES: usize = 100;

/// Rates are stored with the denom and the block height
pub const RATES: Map<(&str, u64), Rates> = Map::new("rates");

#[cw_serde]
pub struct Config {
    pub admin_address: Addr,
}

#[cw_serde]
pub struct Rates {
    pub purchase_rate: Decimal,
    pub redemption_rate: Decimal,
    /// Unix timestamp
    pub update_time: u64,
}
