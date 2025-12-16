use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub token_contract: String,

    // beneficiaries (fixed at genesis)
    pub dev_beneficiary: String,
    pub reserve_beneficiary: String,

    // totals (hard-enforced in contract)
    pub dev_total: Uint128,       // 30,000,000
    pub reserve_total: Uint128,   // 90,000,000
}

#[cw_serde]
pub enum ExecuteMsg {
    // Model 2: anyone can call; funds always go to beneficiary
    Claim { id: String }, // "dev" | "reserve"
    ClaimAll {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(ScheduleResponse)]
    Schedule { id: String },

    #[returns(ClaimableResponse)]
    Claimable { id: String, at_time: Option<u64> },
}

#[cw_serde]
pub struct ConfigResponse {
    pub token_contract: Addr,
    pub genesis_time: u64,
    pub month_seconds: u64,
    pub year_seconds: u64,
}

#[cw_serde]
pub struct ScheduleResponse {
    pub id: String,
    pub beneficiary: Addr,
    pub total: Uint128,
    pub claimed: Uint128,
    pub cliff_seconds: u64,
    pub period_seconds: u64,
    pub period_count: u32,
    pub amount_per_period: Uint128,
    pub remainder: Uint128,
}

#[cw_serde]
pub struct ClaimableResponse {
    pub id: String,
    pub claimable: Uint128,
    pub unlocked: Uint128,
    pub total: Uint128,
    pub claimed: Uint128,
}
