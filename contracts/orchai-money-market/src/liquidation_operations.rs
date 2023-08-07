use cosmwasm_std::{to_binary, Addr, CosmosMsg, Deps, Env, StdResult, Uint128, Uint256, WasmMsg};
use orchai::{liquidation::LiquidationExecuteMsg, querier::query_token_balance};

use crate::state::CONFIG;
use cw20::Cw20ExecuteMsg;

pub fn query_liquidation_submit_bid_msg(
    deps: Deps,
    _env: Env,
    sender: Option<Addr>,
    amount: Option<Uint128>,
    collateral_token: Addr,
    premium_slot: u8,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let stable_addr = deps.api.addr_humanize(&config.stable_addr)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: stable_addr.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(&config.liquidation)?.to_string(),
            amount: amount.unwrap_or_else(|| {
                query_token_balance(&deps.querier, stable_addr, sender.unwrap()).unwrap()
            }),
            msg: to_binary(&LiquidationExecuteMsg::SubmitBid {
                collateral_token: collateral_token.to_string(),
                premium_slot,
            })?,
        })?,
        funds: vec![],
    });

    Ok(vec![msg])
}

pub fn query_liquidation_retract_bid_msg(
    deps: Deps,
    _env: Env,
    bid_idx: Uint128,
    amount: Option<Uint256>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.liquidation)?.to_string(),
        msg: to_binary(&LiquidationExecuteMsg::RetractBid { bid_idx, amount })?,
        funds: vec![],
    });

    Ok(vec![msg])
}

pub fn query_liquidation_activate_bids_msg(
    deps: Deps,
    _env: Env,
    collateral_token: Addr,
    bids_idx: Option<Vec<Uint128>>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.liquidation)?.to_string(),
        msg: to_binary(&LiquidationExecuteMsg::ActivateBids {
            collateral_token: collateral_token.to_string(),
            bids_idx,
        })?,
        funds: vec![],
    });

    Ok(vec![msg])
}

pub fn query_liquidation_claim_liquidations_msg(
    deps: Deps,
    _env: Env,
    collateral_token: Addr,
    bids_idx: Option<Vec<Uint128>>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.liquidation)?.to_string(),
        msg: to_binary(&LiquidationExecuteMsg::ClaimLiquidations {
            collateral_token: collateral_token.to_string(),
            bids_idx,
        })?,
        funds: vec![],
    });

    Ok(vec![msg])
}

pub fn query_liquidation_claim_lending_rewards_msg(
    deps: Deps,
    _env: Env,
    collateral_token: Addr,
    bids_idx: Option<Vec<Uint128>>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.liquidation)?.to_string(),
        msg: to_binary(&LiquidationExecuteMsg::ClaimLendingRewards {
            collateral_token: collateral_token.to_string(),
            bids_idx,
        })?,
        funds: vec![],
    });

    Ok(vec![msg])
}
