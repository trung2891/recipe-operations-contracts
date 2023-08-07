use cosmwasm_std::{to_binary, Addr, CosmosMsg, Deps, Env, StdResult};
use orchai::custody::CustodyExecuteMsg;

use crate::state::read_collateral_info;

pub fn query_custody_claim_rewards_msgs(
    deps: Deps,
    _env: Env,
    collateral: Addr,
    recipient: Option<String>,
) -> StdResult<Vec<CosmosMsg>> {
    let collateral_raw = deps.api.addr_canonicalize(collateral.as_str())?;
    let collateral_info = read_collateral_info(deps.storage, &collateral_raw)?;

    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&collateral_info.custody_contract)?
            .to_string(),
        msg: to_binary(&CustodyExecuteMsg::ClaimRewards { recipient })?,
        funds: vec![],
    });
    Ok(vec![msg])
}
