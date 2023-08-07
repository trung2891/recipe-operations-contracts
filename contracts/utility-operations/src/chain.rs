use std::vec;

use cosmwasm_std::{Addr, Coin, CosmosMsg, Deps, StdResult, Uint128};

pub fn query_delegate_msg(
    deps: Deps,
    executor_addr: Addr,
    validator: String,
    denom: String,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let amount = amount.unwrap_or_else(|| {
        deps.querier
            .query_balance(executor_addr.to_string(), denom.clone())
            .unwrap()
            .amount
    });

    Ok(vec![CosmosMsg::Staking(
        cosmwasm_std::StakingMsg::Delegate {
            validator,
            amount: Coin { denom, amount },
        },
    )])
}

pub fn query_undelegate_msg(
    deps: Deps,
    executor_addr: Addr,
    validator: String,
    denom: String,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    let amount = amount.unwrap_or_else(|| {
        deps.querier
            .query_delegation(executor_addr.to_string(), validator.clone())
            .unwrap()
            .unwrap()
            .amount
            .amount
    });

    Ok(vec![CosmosMsg::Staking(
        cosmwasm_std::StakingMsg::Undelegate {
            validator,
            amount: Coin { denom, amount },
        },
    )])
}
