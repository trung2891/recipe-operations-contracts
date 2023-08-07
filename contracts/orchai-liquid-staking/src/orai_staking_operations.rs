use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Deps, Env, StdError, StdResult, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use orchai::{
    orai_staking::{BondType, OraiStakingExecuteMsg},
    querier::query_token_balance,
};

use crate::state::CONFIG;

pub fn query_orai_staking_bond_msg(
    deps: Deps,
    _env: Env,
    sender: Option<Addr>,
    bond_type: BondType,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    if sender.is_none() && amount.is_none() {
        return Err(StdError::GenericErr {
            msg: "Sender and amount is none!!!".to_string(),
        });
    }
    let config = CONFIG.load(deps.storage)?;
    let orai_staking_hub = deps.api.addr_humanize(&config.orai_staking_hub)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: orai_staking_hub.to_string(),
        msg: to_binary(&OraiStakingExecuteMsg::Bond { bond_type })?,
        funds: vec![Coin {
            denom: config.orai_staking_denom.clone(),
            amount: amount.unwrap_or_else(|| {
                deps.querier
                    .query_balance(sender.unwrap().to_string(), config.orai_staking_denom)
                    .unwrap()
                    .amount
            }),
        }],
    });

    Ok(vec![msg])
}

pub fn query_orai_staking_unbond_msg(
    deps: Deps,
    _env: Env,
    executor_addr: Addr,
    sender: Addr,
    token: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let orai_staking_hub = deps.api.addr_humanize(&config.orai_staking_hub)?;
    let mut messages: Vec<CosmosMsg> = vec![];
    let amount = amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, token.clone(), sender.clone()).unwrap()
    });

    if sender != executor_addr {
        messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: token.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: sender.to_string(),
                recipient: executor_addr.to_string(),
                amount,
            })?,
            funds: vec![],
        }))
    }

    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: orai_staking_hub.to_string(),
            amount,
            msg: to_binary(&OraiStakingExecuteMsg::Unbond {})?,
        })?,
        funds: vec![],
    }));

    Ok(messages)
}

pub fn query_orai_staking_claim_rewards_msg(
    deps: Deps,
    _env: Env,
    recipient: Option<String>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&config.orai_staking_reward)?
            .to_string(),
        msg: to_binary(&OraiStakingExecuteMsg::ClaimRewards { recipient })?,
        funds: vec![],
    });

    Ok(vec![msg])
}

pub fn query_orai_staking_convert_msg(
    deps: Deps,
    _env: Env,
    executor_addr: Addr,
    sender: Addr,
    from_token: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let orai_staking_hub = deps.api.addr_humanize(&config.orai_staking_hub)?;

    let amount = amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, from_token.clone(), sender.clone()).unwrap()
    });

    let mut messages: Vec<CosmosMsg> = vec![];

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
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: from_token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: orai_staking_hub.to_string(),
            amount,
            msg: to_binary(&OraiStakingExecuteMsg::Convert {})?,
        })?,
        funds: vec![],
    }));

    Ok(messages)
}
