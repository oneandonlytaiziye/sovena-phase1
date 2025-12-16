use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::Uint128;

use sovena_lock::contract::instantiate;
use sovena_lock::msg::InstantiateMsg;

#[test]
fn instantiate_requires_fixed_totals() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("anyone", &[]);

    let bad = InstantiateMsg {
        token_contract: "secret1tokenxxxxxxxxxxxxxxxxxxxxxxxxxxxx".into(),
        dev_beneficiary: "secret1devxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".into(),
        reserve_beneficiary: "secret1resxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".into(),
        dev_total: Uint128::new(1),
        reserve_total: Uint128::new(90_000_000),
    };

    let err = instantiate(deps.as_mut(), env, info, bad).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("dev_total must be 30,000,000"));
}
