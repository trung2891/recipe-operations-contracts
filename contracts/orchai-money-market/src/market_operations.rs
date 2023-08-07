use cosmwasm_std::{to_binary, Addr, CosmosMsg, Deps, Env, StdResult, Uint128, Uint256};
use orchai::{
    market::{AStableRewardExecuteMsg, MarketExecuteMsg},
    querier::query_token_balance,
};

use crate::state::CONFIG;
use cw20::Cw20ExecuteMsg;

pub fn query_market_borrow_stable_msgs(
    deps: Deps,
    _env: Env,
    borrow_amount: Uint256,
    to: Option<String>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.market)?.to_string(),
        msg: to_binary(&MarketExecuteMsg::BorrowStable { borrow_amount, to })?,
        funds: vec![],
    });
    Ok(vec![msg])
}

pub fn query_market_deposit_stable_msgs(
    deps: Deps,
    _env: Env,
    executor_addr: Addr,
    sender: Addr,
    stable_amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let stable_addr = deps.api.addr_humanize(&config.stable_addr)?;
    let mut messages: Vec<CosmosMsg> = vec![];

    let stable_amount = stable_amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, stable_addr.clone(), sender.clone()).unwrap()
    });

    if sender != executor_addr {
        messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: stable_addr.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: sender.to_string(),
                recipient: executor_addr.to_string(),
                amount: stable_amount,
            })?,
            funds: vec![],
        }))
    }

    messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: stable_addr.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(&config.market)?.to_string(),
            amount: stable_amount,
            msg: to_binary(&MarketExecuteMsg::DepositStable {})?,
        })?,
        funds: vec![],
    }));
    Ok(messages)
}

pub fn query_market_redeem_stable_msgs(
    deps: Deps,
    _env: Env,
    executor_addr: Addr,
    sender: Addr,
    a_stable_amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let a_stable_addr = deps.api.addr_humanize(&config.a_stable_contract)?;
    let mut messages: Vec<CosmosMsg> = vec![];

    let a_stable_amount = a_stable_amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, a_stable_addr.clone(), sender.clone()).unwrap()
    });

    if sender != executor_addr {
        messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: a_stable_addr.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: sender.to_string(),
                recipient: executor_addr.to_string(),
                amount: a_stable_amount,
            })?,
            funds: vec![],
        }))
    }

    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: a_stable_addr.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(&config.market)?.to_string(),
            amount: a_stable_amount,
            msg: to_binary(&MarketExecuteMsg::RedeemStable {})?,
        })?,
        funds: vec![],
    });
    Ok(vec![msg])
}

pub fn query_market_repay_stable_msgs(
    deps: Deps,
    _env: Env,
    amount: Uint128,
    executor_addr: Addr,
    sender: Addr,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let stable_addr = deps.api.addr_humanize(&config.stable_addr)?;

    let mut messages: Vec<CosmosMsg> = vec![];
    if sender != executor_addr {
        messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: stable_addr.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: sender.to_string(),
                recipient: executor_addr.to_string(),
                amount,
            })?,
            funds: vec![],
        }))
    }

    messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.stable_addr)?.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(&config.market)?.to_string(),
            amount,
            msg: to_binary(&MarketExecuteMsg::RepayStable {})?,
        })?,
        funds: vec![],
    }));
    Ok(messages)
}

pub fn query_market_repay_stable_for_msgs(
    deps: Deps,
    _env: Env,
    amount: Uint128,
    borrower: String,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.stable_addr)?.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(&config.market)?.to_string(),
            amount,
            msg: to_binary(&MarketExecuteMsg::RepayStableFor { borrower })?,
        })?,
        funds: vec![],
    });
    Ok(vec![msg])
}

pub fn query_market_claim_borrower_rewards_msgs(
    deps: Deps,
    _env: Env,
    to: Option<String>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.market)?.to_string(),
        msg: to_binary(&MarketExecuteMsg::ClaimRewards { to })?,
        funds: vec![],
    });
    Ok(vec![msg])
}

pub fn query_market_claim_lender_rewards_msgs(
    deps: Deps,
    _env: Env,
    to: Option<String>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&config.a_stable_contract_reward)?
            .to_string(),
        msg: to_binary(&AStableRewardExecuteMsg::ClaimRewards { recipient: to })?,
        funds: vec![],
    });
    Ok(vec![msg])
}
