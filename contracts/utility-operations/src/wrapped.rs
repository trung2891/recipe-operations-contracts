use std::vec;

use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Deps, StdResult, Uint128};
use cw20::Cw20ExecuteMsg;
use utility::{querier::query_token_balance, wrapped::WrappedExecuteMsg};

use crate::state::CONFIG;

pub fn query_wrapped_convert_to_denom(
    deps: Deps,
    from_token: Addr,
    target_denom: String,
    executor_addr: Addr,
    sender: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let wrapped_contract = deps.api.addr_humanize(&config.wrapped_contract)?;

    let mut messages: Vec<CosmosMsg> = vec![];
    let amount = amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, from_token.clone(), sender.clone()).unwrap()
    });

    if sender != executor_addr {
        messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: from_token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: sender.to_string(),
                recipient: executor_addr.to_string(),
                amount,
            })?,
            funds: vec![],
        }))
    }

    messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: from_token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: wrapped_contract.to_string(),
            amount,
            msg: to_binary(&WrappedExecuteMsg::ConvertToDenom { target_denom })?,
        })?,
        funds: vec![],
    }));

    Ok(messages)
}

pub fn query_wrapped_convert_to_cw20(
    deps: Deps,
    from_denom: String,
    executor_addr: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let wrapped_contract = deps.api.addr_humanize(&config.wrapped_contract)?;

    let amount = amount.unwrap_or_else(|| {
        deps.querier
            .query_balance(executor_addr.to_string(), from_denom.clone())
            .unwrap()
            .amount
    });

    Ok(vec![CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: wrapped_contract.to_string(),
        msg: to_binary(&WrappedExecuteMsg::ConvertToCw20 {})?,
        funds: vec![Coin {
            denom: from_denom,
            amount,
        }],
    })])
}
