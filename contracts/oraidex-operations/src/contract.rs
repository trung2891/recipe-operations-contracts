#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg,
};
use oraidex::asset::{Asset, AssetInfo};

use oraidex::pair::PairExecuteMsg;
use oraidex::querier::{query_staking_amount, query_token_balance};
use oraidex::router::{OraiswapExecuteMsg, SwapOperation};
use oraidex::staking::StakingExecuteMsg;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, Operations, QueryMsg};
use crate::state::{Config, CONFIG};

use cw20::Cw20ExecuteMsg;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:smart-wallet";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CONFIG.save(
        deps.storage,
        &Config {
            owner: deps.api.addr_canonicalize(msg.owner.as_str())?,
            oraiswap_router: deps.api.addr_canonicalize(msg.oraiswap_router.as_str())?,
            oraiswap_staking: deps.api.addr_canonicalize(msg.oraiswap_staking.as_str())?,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            oraiswap_router,
            oraiswap_staking,
        } => execute_update_config(deps, env, info, owner, oraiswap_router, oraiswap_staking),
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<Addr>,
    oraiswap_router: Option<Addr>,
    oraiswap_staking: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if deps.api.addr_humanize(&config.owner)? != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }

    if let Some(oraiswap_router) = oraiswap_router {
        config.oraiswap_router = deps.api.addr_canonicalize(oraiswap_router.as_str())?;
    }

    if let Some(oraiswap_staking) = oraiswap_staking {
        config.oraiswap_staking = deps.api.addr_canonicalize(oraiswap_staking.as_str())?;
    }
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Messages { msg } => match from_binary(&msg) {
            Ok(Operations::SwapOperations {
                executor_addr,
                sender,
                amount,
                operations,
                minimum_receive,
                to,
            }) => to_binary(&query_swap_operations_msg(
                deps,
                env,
                executor_addr,
                sender,
                amount,
                operations,
                minimum_receive,
                to,
            )?),
            Ok(Operations::ProvideLiquidity {
                pair_contract,
                assets,
                slippage_tolerance,
                receiver,
            }) => to_binary(&query_provide_liquidity_msg(
                deps,
                env,
                pair_contract,
                assets,
                slippage_tolerance,
                receiver,
            )?),
            Ok(Operations::WithdrawLiquidity {
                sender,
                pair_contract,
                lp_token,
                amount,
            }) => to_binary(&query_withdraw_liquidity_msg(
                deps,
                env,
                sender,
                pair_contract,
                lp_token,
                amount,
            )?),
            Ok(Operations::Bond {
                sender,
                lp_token,
                asset_info,
                amount,
            }) => to_binary(&query_bond_msg(
                deps, env, sender, lp_token, asset_info, amount,
            )?),
            Ok(Operations::Unbond {
                sender,
                asset_info,
                amount,
            }) => to_binary(&query_unbond_msg(deps, env, sender, asset_info, amount)?),
            Ok(Operations::Withdraw { asset_info }) => {
                to_binary(&query_withdraw_msg(deps, env, asset_info)?)
            }
            _ => Err(StdError::NotFound {
                kind: "Messages not found".to_string(),
            }),
        },
        QueryMsg::Config {} => to_binary(&query_config(deps, env)?),
    }
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?,
        oraiswap_router: deps.api.addr_humanize(&config.oraiswap_router)?,
        oraiswap_staking: deps.api.addr_humanize(&config.oraiswap_staking)?,
    })
}

pub fn query_bond_msg(
    deps: Deps,
    _env: Env,
    sender: Option<Addr>,
    lp_token: Addr,
    asset_info: AssetInfo,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    if sender.is_none() && amount.is_none() {
        return Err(StdError::GenericErr {
            msg: "Sender and amount is none!!!".to_string(),
        });
    }

    let amount = amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, lp_token.clone(), sender.unwrap()).unwrap()
    });

    let config = CONFIG.load(deps.storage)?;
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: lp_token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps
                .api
                .addr_humanize(&config.oraiswap_staking)?
                .to_string(),
            amount,
            msg: to_binary(&StakingExecuteMsg::Bond { asset_info })?,
        })?,
        funds: vec![],
    });
    Ok(vec![msg])
}

pub fn query_unbond_msg(
    deps: Deps,
    _env: Env,
    sender: Option<Addr>,
    asset_info: AssetInfo,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    if sender.is_none() && amount.is_none() {
        return Err(StdError::GenericErr {
            msg: "Sender and amount is none!!!".to_string(),
        });
    }
    let config = CONFIG.load(deps.storage)?;

    let amount = amount.unwrap_or_else(|| {
        query_staking_amount(
            &deps.querier,
            deps.api.addr_humanize(&config.oraiswap_staking).unwrap(),
            sender.unwrap(),
            asset_info.clone(),
        )
        .unwrap()
    });
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&config.oraiswap_staking)?
            .to_string(),
        msg: to_binary(&StakingExecuteMsg::Unbond { asset_info, amount })?,
        funds: vec![],
    });
    Ok(vec![msg])
}

pub fn query_withdraw_msg(
    deps: Deps,
    _env: Env,
    asset_info: Option<AssetInfo>,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps
            .api
            .addr_humanize(&config.oraiswap_staking)?
            .to_string(),
        msg: to_binary(&StakingExecuteMsg::Withdraw { asset_info })?,
        funds: vec![],
    });
    Ok(vec![msg])
}

pub fn query_withdraw_liquidity_msg(
    deps: Deps,
    _env: Env,
    sender: Option<Addr>,
    pair_contract: Addr,
    lp_token: Addr,
    amount: Option<Uint128>,
) -> StdResult<Vec<CosmosMsg>> {
    if sender.is_none() && amount.is_none() {
        return Err(StdError::GenericErr {
            msg: "Sender and amount is none!!!".to_string(),
        });
    }

    let amount = amount.unwrap_or_else(|| {
        query_token_balance(&deps.querier, lp_token.clone(), sender.unwrap()).unwrap()
    });
    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: lp_token.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: pair_contract.to_string(),
            amount,
            msg: to_binary(&PairExecuteMsg::WithdrawLiquidity {})?,
        })?,
        funds: vec![],
    });
    Ok(vec![msg])
}
pub fn query_provide_liquidity_msg(
    _deps: Deps,
    _env: Env,
    pair_contract: Addr,
    assets: [Asset; 2],
    slippage_tolerance: Option<Decimal>,
    receiver: Option<Addr>,
) -> StdResult<Vec<CosmosMsg>> {
    let mut funds: Vec<Coin> = vec![];
    for asset in &assets {
        if let AssetInfo::NativeToken { denom } = &asset.info {
            funds.push(Coin {
                denom: denom.clone(),
                amount: asset.amount,
            });
        }
    }

    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: pair_contract.to_string(),
        msg: to_binary(&PairExecuteMsg::ProvideLiquidity {
            assets,
            slippage_tolerance,
            receiver,
        })?,
        funds,
    });
    Ok(vec![msg])
}
pub fn query_swap_operations_msg(
    deps: Deps,
    _env: Env,
    executor_addr: Addr,
    sender: Addr,
    amount: Option<Uint128>,
    operations: Vec<SwapOperation>,
    minimum_receive: Option<Uint128>,
    to: Option<Addr>,
) -> StdResult<Vec<CosmosMsg>> {
    if operations.len() == 0 {
        return Err(StdError::GenericErr {
            msg: "Swap operations is empty!!!".to_string(),
        });
    }

    let config = CONFIG.load(deps.storage)?;
    let swap_router = deps.api.addr_humanize(&config.oraiswap_router)?;
    let offer_asset = operations[0].get_offer_asset_info();

    let mut messages: Vec<CosmosMsg> = vec![];

    match offer_asset.clone() {
        AssetInfo::NativeToken { denom } => {
            let amount = amount.unwrap_or_else(|| {
                deps.querier
                    .query_balance(sender.to_string(), denom.clone())
                    .unwrap()
                    .amount
            });
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: swap_router.to_string(),
                msg: to_binary(&OraiswapExecuteMsg::ExecuteSwapOperations {
                    operations,
                    minimum_receive,
                    to,
                })?,
                funds: vec![Coin { denom, amount }],
            }));
        }

        AssetInfo::Token { contract_addr } => {
            let amount = amount.unwrap_or_else(|| {
                query_token_balance(&deps.querier, contract_addr.clone(), sender.clone()).unwrap()
            });

            if sender != executor_addr {
                messages.push(CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                    contract_addr: contract_addr.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                        owner: sender.to_string(),
                        recipient: executor_addr.to_string(),
                        amount,
                    })?,
                    funds: vec![],
                }))
            }
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Send {
                    contract: swap_router.to_string(),
                    amount,
                    msg: to_binary(&OraiswapExecuteMsg::ExecuteSwapOperations {
                        operations,
                        minimum_receive,
                        to,
                    })?,
                })?,
                funds: vec![],
            }));
        }
    };
    Ok(messages)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
