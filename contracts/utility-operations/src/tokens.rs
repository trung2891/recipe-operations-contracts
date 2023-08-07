use cosmwasm_std::{to_binary, Addr, Binary, CosmosMsg, Deps, StdError, StdResult, Uint128};
use cw20::{BalanceResponse, Cw20ExecuteMsg, Cw20QueryMsg};

pub fn query_token_transfer_msg(
    deps: Deps,
    token: Addr,
    sender: Option<Addr>,
    recipient: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    if sender.is_none() && amount.is_none() {
        return Err(StdError::GenericErr {
            msg: "Sender and amount is none!!!".to_string(),
        });
    }

    let amount = amount.unwrap_or_else(|| {
        let balance: BalanceResponse = deps
            .querier
            .query_wasm_smart(
                token.to_string(),
                &Cw20QueryMsg::Balance {
                    address: sender.unwrap().to_string(),
                },
            )
            .unwrap();

        balance.balance
    });

    Ok(vec![CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: recipient.to_string(),
            amount,
        })?,
        funds: vec![],
    })])
}

pub fn query_token_transfer_from_msg(
    deps: Deps,
    token: Addr,
    owner: Addr,
    recipient: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let amount = amount.unwrap_or_else(|| {
        let balance: BalanceResponse = deps
            .querier
            .query_wasm_smart(
                token.to_string(),
                &Cw20QueryMsg::Balance {
                    address: owner.to_string(),
                },
            )
            .unwrap();

        balance.balance
    });
    Ok(vec![CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
            owner: owner.to_string(),
            recipient: recipient.to_string(),
            amount,
        })?,
        funds: vec![],
    })])
}

pub fn query_token_send_msg(
    deps: Deps,
    token: Addr,
    sender: Option<Addr>,
    contract: Addr,
    amount: Option<Uint128>,
    msg: Binary,
) -> StdResult<Vec<CosmosMsg>> {
    if sender.is_none() && amount.is_none() {
        return Err(StdError::GenericErr {
            msg: "Sender and amount is none!!!".to_string(),
        });
    }

    let amount = amount.unwrap_or_else(|| {
        let balance: BalanceResponse = deps
            .querier
            .query_wasm_smart(
                token.to_string(),
                &Cw20QueryMsg::Balance {
                    address: sender.unwrap().to_string(),
                },
            )
            .unwrap();

        balance.balance
    });

    Ok(vec![CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: contract.to_string(),
            amount,
            msg,
        })?,
        funds: vec![],
    })])
}

pub fn query_token_send_from_msg(
    deps: Deps,
    token: Addr,
    owner: Addr,
    contract: Addr,
    amount: Option<Uint128>,
    msg: Binary,
) -> StdResult<Vec<CosmosMsg>> {
    let amount = amount.unwrap_or_else(|| {
        let balance: BalanceResponse = deps
            .querier
            .query_wasm_smart(
                token.to_string(),
                &Cw20QueryMsg::Balance {
                    address: owner.to_string(),
                },
            )
            .unwrap();

        balance.balance
    });
    Ok(vec![CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::SendFrom {
            owner: owner.to_string(),
            contract: contract.to_string(),
            amount,
            msg,
        })?,
        funds: vec![],
    })])
}
