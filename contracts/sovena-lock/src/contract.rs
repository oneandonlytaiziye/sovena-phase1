use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;

const MONTH_SECONDS: u64 = 30 * 24 * 60 * 60;
const YEAR_SECONDS: u64 = 365 * 24 * 60 * 60;

fn calc_unlocked(s: &Schedule, now: u64) -> Uint128 {
    let start = s.start_time.saturating_add(s.cliff_seconds);
    if now < start {
        return Uint128::zero();
    }

    let elapsed = now - start;
    let mut periods = (elapsed / s.period_seconds) as u32 + 1;
    if periods > s.period_count {
        periods = s.period_count;
    }

    let mut unlocked = s
        .amount_per_period
        .checked_mul(Uint128::from(periods as u128))
        .unwrap_or(Uint128::zero());

    if periods == s.period_count {
        unlocked = unlocked.saturating_add(s.remainder);
    }

    if unlocked > s.total { s.total } else { unlocked }
}

fn calc_claimable(s: &Schedule, now: u64) -> (Uint128, Uint128) {
    let unlocked = calc_unlocked(s, now);
    let claimable = unlocked.saturating_sub(s.claimed);
    (claimable, unlocked)
}

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, _info: MessageInfo, msg: InstantiateMsg) -> Result<Response, ContractError> {
    let token_contract = deps.api.addr_validate(&msg.token_contract)?;
    let dev_beneficiary = deps.api.addr_validate(&msg.dev_beneficiary)?;
    let reserve_beneficiary = deps.api.addr_validate(&msg.reserve_beneficiary)?;

    // hard discipline: totals must match the constitution
    if msg.dev_total != Uint128::new(30_000_000) {
        return Err(ContractError::InvalidConfig{ reason: "dev_total must be 30,000,000".into() });
    }
    if msg.reserve_total != Uint128::new(90_000_000) {
        return Err(ContractError::InvalidConfig{ reason: "reserve_total must be 90,000,000".into() });
    }

    let genesis_time = env.block.time.seconds();

    CONFIG.save(deps.storage, &Config {
        token_contract,
        genesis_time,
        month_seconds: MONTH_SECONDS,
        year_seconds: YEAR_SECONDS,
    })?;

    // DEV: 3y cliff + 7 yearly tranches, remainder=2
    let dev = Schedule {
        id: "dev".into(),
        beneficiary: dev_beneficiary,
        total: msg.dev_total,
        claimed: Uint128::zero(),
        start_time: genesis_time,
        cliff_seconds: 3 * YEAR_SECONDS,
        period_seconds: YEAR_SECONDS,
        period_count: 7,
        amount_per_period: Uint128::new(4_285_714),
        remainder: Uint128::new(2),
    };

    // RESERVE: 120 monthly tranches
    let reserve = Schedule {
        id: "reserve".into(),
        beneficiary: reserve_beneficiary,
        total: msg.reserve_total,
        claimed: Uint128::zero(),
        start_time: genesis_time,
        cliff_seconds: 0,
        period_seconds: MONTH_SECONDS,
        period_count: 120,
        amount_per_period: Uint128::new(750_000),
        remainder: Uint128::zero(),
    };

    SCHEDULES.save(deps.storage, "dev", &dev)?;
    SCHEDULES.save(deps.storage, "reserve", &reserve)?;

    Ok(Response::new()
        .add_attribute("contract", "sovena-lock")
        .add_attribute("action", "instantiate")
        .add_attribute("genesis_time", genesis_time.to_string()))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, _info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Claim { id } => exec_claim(deps, env, id),
        ExecuteMsg::ClaimAll {} => {
            // best-effort; claim if available
            let a = exec_claim(deps.branch(), env.clone(), "dev".into()).ok();
            let b = exec_claim(deps, env, "reserve".into()).ok();

            let mut resp = Response::new().add_attribute("action", "claim_all");
            if let Some(r) = a { resp = resp.merge(r); }
            if let Some(r) = b { resp = resp.merge(r); }
            Ok(resp)
        }
    }
}

fn exec_claim(deps: DepsMut, env: Env, id: String) -> Result<Response, ContractError> {
    let key = id.as_str();
    let mut s = SCHEDULES.may_load(deps.storage, key)?.ok_or(ContractError::ScheduleNotFound {})?;

    let now = env.block.time.seconds();
    let (claimable, _unlocked) = calc_claimable(&s, now);
    if claimable.is_zero() {
        return Err(ContractError::NothingToClaim {});
    }

    s.claimed = s.claimed.saturating_add(claimable);
    SCHEDULES.save(deps.storage, key, &s)?;

    let cfg = CONFIG.load(deps.storage)?;

    // SNIP-20 transfer msg (standard pattern)
    let transfer_msg = to_json_binary(&serde_json::json!({
        "transfer": { "recipient": s.beneficiary.to_string(), "amount": claimable.to_string() }
    }))?;

    let wasm = WasmMsg::Execute {
        contract_addr: cfg.token_contract.to_string(),
        msg: transfer_msg,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(wasm)
        .add_attribute("action", "claim")
        .add_attribute("id", id)
        .add_attribute("to", s.beneficiary.to_string())
        .add_attribute("amount", claimable.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => {
            let c = CONFIG.load(deps.storage)?;
            to_json_binary(&ConfigResponse {
                token_contract: c.token_contract,
                genesis_time: c.genesis_time,
                month_seconds: c.month_seconds,
                year_seconds: c.year_seconds,
            })
        }
        QueryMsg::Schedule { id } => {
            let s = SCHEDULES.load(deps.storage, id.as_str())?;
            to_json_binary(&ScheduleResponse {
                id: s.id,
                beneficiary: s.beneficiary,
                total: s.total,
                claimed: s.claimed,
                cliff_seconds: s.cliff_seconds,
                period_seconds: s.period_seconds,
                period_count: s.period_count,
                amount_per_period: s.amount_per_period,
                remainder: s.remainder,
            })
        }
        QueryMsg::Claimable { id, at_time } => {
            let s = SCHEDULES.load(deps.storage, id.as_str())?;
            let now = at_time.unwrap_or_else(|| env.block.time.seconds());
            let (claimable, unlocked) = calc_claimable(&s, now);
            to_json_binary(&ClaimableResponse {
                id: s.id,
                claimable,
                unlocked,
                total: s.total,
                claimed: s.claimed,
            })
        }
    }
}
