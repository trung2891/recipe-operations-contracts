use std::str::FromStr;

use cosmwasm_std::{to_binary, Addr, CosmosMsg, Deps, Env, StdError, StdResult, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
use orchai::{
    custody::CustodyExecuteMsg,
    overseer::OverseerExecuteMsg,
    querier::{query_custody_borrower, query_token_balance},
};

use crate::state::{read_collateral_info, CONFIG};

pub fn query_overseer_provide_and_lock_collateral_msg(
    deps: Deps,
    _env: Env,
    executor_addr: Addr,
    sender: Addr,
    collateral: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;

    let mut messages: Vec<CosmosMsg> = vec![];

    let collateral_raw = deps.api.addr_canonicalize(collateral.as_str())?;
    let collateral_info = read_collateral_info(deps.storage, &collateral_raw)?;

    let amount = amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, collateral.clone(), sender.clone()).unwrap()
    });

    if sender != executor_addr {
        messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: collateral.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: sender.to_string(),
                recipient: executor_addr.to_string(),
                amount,
            })?,
            funds: vec![],
        }))
    }

    // add deposit collateral msg
    messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: collateral.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps
                .api
                .addr_humanize(&collateral_info.custody_contract)?
                .to_string(),
            amount,
            msg: to_binary(&CustodyExecuteMsg::DepositCollateral {})?,
        })?,
        funds: vec![],
    }));

    // add lock collateral msg
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.overseer)?.to_string(),
        msg: to_binary(&OverseerExecuteMsg::LockCollateral {
            collaterals: vec![(collateral.to_string(), amount.into())],
        })?,
        funds: vec![],
    }));

    Ok(messages)
}

pub fn query_overseer_unlock_and_withdraw_collateral_msg(
    deps: Deps,
    _env: Env,
    sender: Option<Addr>,
    collateral: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;

    if sender.is_none() && amount.is_none() {
        return Err(StdError::GenericErr {
            msg: "Sender and amount is none!!!".to_string(),
        });
    }
    let mut messages: Vec<CosmosMsg> = vec![];

    let collateral_raw = deps.api.addr_canonicalize(collateral.as_str())?;
    let collateral_info = read_collateral_info(deps.storage, &collateral_raw)?;

    // add unlock_collateral msg
    let amount = amount.unwrap_or_else(|| {
        let borrower_info = query_custody_borrower(
            &deps.querier,
            deps.api
                .addr_humanize(&collateral_info.custody_contract)
                .unwrap(),
            sender.unwrap(),
        )
        .unwrap();
        return Uint128::from_str(&(borrower_info.balance - borrower_info.spendable).to_string())
            .unwrap();
    });
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.overseer)?.to_string(),
        msg: to_binary(&OverseerExecuteMsg::UnlockCollateral {
            collaterals: vec![(collateral.to_string(), amount.into())],
        })?,
        funds: vec![],
    }));

    // push withdraw_collateral msg
    messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&collateral_info.custody_contract)?
            .to_string(),
        msg: to_binary(&CustodyExecuteMsg::WithdrawCollateral {
            amount: Some(amount.into()),
        })?,
        funds: vec![],
    }));

    Ok(messages)
}
