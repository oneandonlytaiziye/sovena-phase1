use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Config {
    pub token_contract: Addr,
    pub genesis_time: u64,      // set at instantiate (mainnet deploy time)
    pub month_seconds: u64,     // 30 days fixed
    pub year_seconds: u64,      // 365 days fixed
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Schedule {
    pub id: String,                 // "dev" | "reserve"
    pub beneficiary: Addr,
    pub total: Uint128,
    pub claimed: Uint128,

    pub start_time: u64,            // genesis_time
    pub cliff_seconds: u64,         // dev: 3y, reserve: 0
    pub period_seconds: u64,        // dev: 1y, reserve: 1m
    pub period_count: u32,          // dev: 7, reserve: 120
    pub amount_per_period: Uint128, // dev: 4,285,714 ; reserve: 750,000
    pub remainder: Uint128,         // dev: 2 ; reserve: 0
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const SCHEDULES: Map<&str, Schedule> = Map::new("schedules");
