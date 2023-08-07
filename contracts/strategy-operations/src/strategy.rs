use cosmwasm_std::{to_binary, Addr, CosmosMsg, Deps, StdResult};
use strategy::strategy::StrategyExecuteMsg;

use crate::state::CONFIG;

pub fn query_verify_strategy_msg(
    deps: Deps,
    creator: Addr,
    id: String,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;

    Ok(vec![CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&config.strategy_contract)?
            .to_string(),
        msg: to_binary(&StrategyExecuteMsg::VerifyStrategy { creator, id })?,
        funds: vec![],
    })])
}
