# Milkyway Oracle Contract

## Overview
## Pushing redemption rate and purchase rate

## Transactions
```rust
pub struct InstantiateMsg {
    pub admin_address: String,
}

pub enum ExecuteMsg {
    PostRates {
        /// The purchase rate to save
        purchase_rate: String,

        /// The redemption rate to save
        redemption_rate: String,

        /// Update time
        update_time: u64,

        /// Block height
        block_height: u64,

        /// Denom
        denom: string,
    },
}
```

## Queries
```rust
pub enum QueryMsg {
    /// Returns the contract's config
    #[returns(crate::state::Config)]
    Config {},

    /// Returns the latest redemption rate
    #[returns(RedemptionRateResponse)]
    RedemptionRate {
        denom: String,
        params: Option<Binary>,
    },

    /// Returns historical redemption rates (maximum 100)
    #[returns(RedemptionRatesResponse)]
    RedemptionRates {
        denom: String,
        params: Option<Binary>,
        limit: Option<u64>,
    },

    /// Returns the latest purchase rate
    #[returns(PurchaseRateResponse)]
    PurchaseRate {
        denom: String,
        params: Option<Binary>,
    },

    /// Returns historical purchase rates (maximum 100)
    #[returns(PurchaseRatesResponse)]
    PurchaseRates {
        denom: String,
        params: Option<Binary>,
        limit: Option<u64>,
    },
}
```