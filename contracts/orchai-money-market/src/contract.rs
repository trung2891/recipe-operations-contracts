#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
// use cw2::set_contract_version;

use crate::custody_operations::query_custody_claim_rewards_msgs;
use crate::error::ContractError;
use crate::liquidation_operations::{
    query_liquidation_activate_bids_msg, query_liquidation_claim_lending_rewards_msg,
    query_liquidation_claim_liquidations_msg, query_liquidation_retract_bid_msg,
    query_liquidation_submit_bid_msg,
};
use crate::market_operations::{
    query_market_borrow_stable_msgs, query_market_claim_borrower_rewards_msgs,
    query_market_claim_lender_rewards_msgs, query_market_deposit_stable_msgs,
    query_market_redeem_stable_msgs, query_market_repay_stable_for_msgs,
    query_market_repay_stable_msgs,
};
use crate::msg::{
    CollateralInfoResponse, ConfigResponse, CustodyOperations, ExecuteMsg, InstantiateMsg,
    LiquidationOperations, MarketOperations, MigrateMsg, OverseerOperations, QueryMsg,
};

use crate::overseer_operations::{
    query_overseer_provide_and_lock_collateral_msg,
    query_overseer_unlock_and_withdraw_collateral_msg,
};
use crate::state::{read_collateral_info, store_collateral_info, CollateralInfo, Config, CONFIG};

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
            market: deps.api.addr_canonicalize(msg.market.as_str())?,
            overseer: deps.api.addr_canonicalize(msg.overseer.as_str())?,
            liquidation: deps.api.addr_canonicalize(msg.liquidation.as_str())?,
            stable_addr: deps.api.addr_canonicalize(msg.stable_addr.as_str())?,
            a_stable_contract: deps.api.addr_canonicalize(msg.a_stable_contract.as_str())?,
            a_stable_contract_reward: deps
                .api
                .addr_canonicalize(msg.a_stable_contract_reward.as_str())?,
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
            overseer,
            market,
            liquidation,
            stable_addr,
            a_stable_contract,
            a_stable_contract_reward,
        } => execute_update_config(
            deps,
            env,
            info,
            owner,
            overseer,
            market,
            liquidation,
            stable_addr,
            a_stable_contract,
            a_stable_contract_reward,
        ),
        ExecuteMsg::RegisterCollateral {
            collateral,
            custody_contract,
        } => execute_register_collateral(deps, env, info, collateral, custody_contract),
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<Addr>,
    overseer: Option<Addr>,
    market: Option<Addr>,
    liquidation: Option<Addr>,
    stable_addr: Option<Addr>,
    a_stable_contract: Option<Addr>,
    a_stable_contract_reward: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if deps.api.addr_humanize(&config.owner)? != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }
    if let Some(overseer) = overseer {
        config.overseer = deps.api.addr_canonicalize(overseer.as_str())?;
    }
    if let Some(market) = market {
        config.market = deps.api.addr_canonicalize(market.as_str())?;
    }
    if let Some(liquidation) = liquidation {
        config.liquidation = deps.api.addr_canonicalize(liquidation.as_str())?;
    }
    if let Some(stable_addr) = stable_addr {
        config.stable_addr = deps.api.addr_canonicalize(stable_addr.as_str())?;
    }
    if let Some(a_stable_contract) = a_stable_contract {
        config.a_stable_contract = deps.api.addr_canonicalize(a_stable_contract.as_str())?;
    }
    if let Some(a_stable_contract_reward) = a_stable_contract_reward {
        config.a_stable_contract_reward = deps
            .api
            .addr_canonicalize(a_stable_contract_reward.as_str())?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
pub fn execute_register_collateral(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collateral: Addr,
    custody_contract: Addr,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if deps.api.addr_humanize(&config.owner)? != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let collateral_raw = deps.api.addr_canonicalize(collateral.as_str())?;
    store_collateral_info(
        deps.storage,
        &collateral_raw,
        &CollateralInfo {
            collateral: collateral_raw.clone(),
            custody_contract: deps
                .api
                .addr_canonicalize(custody_contract.as_str())
                .unwrap(),
        },
    )?;

    Ok(Response::new().add_attributes(vec![("action", "register_collateral")]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps, env)?),
        QueryMsg::CollateralInfo { collateral } => {
            to_binary(&query_collateral_info(deps, env, collateral)?)
        }
        QueryMsg::Messages { msg } => match from_binary(&msg) {
            Ok(MarketOperations::BorrowStable { borrow_amount, to }) => to_binary(
                &query_market_borrow_stable_msgs(deps, env, borrow_amount, to)?,
            ),
            Ok(MarketOperations::DepositStable {
                executor_addr,
                sender,
                stable_amount,
            }) => to_binary(&query_market_deposit_stable_msgs(
                deps,
                env,
                executor_addr,
                sender,
                stable_amount,
            )?),
            Ok(MarketOperations::RedeemStable {
                executor_addr,
                sender,
                a_stable_amount,
            }) => to_binary(&query_market_redeem_stable_msgs(
                deps,
                env,
                executor_addr,
                sender,
                a_stable_amount,
            )?),
            Ok(MarketOperations::RepayStable {
                amount,
                executor_addr,
                sender,
            }) => to_binary(&query_market_repay_stable_msgs(
                deps,
                env,
                amount,
                executor_addr,
                sender,
            )?),
            Ok(MarketOperations::RepayStableFor { amount, borrower }) => to_binary(
                &query_market_repay_stable_for_msgs(deps, env, amount, borrower)?,
            ),
            Ok(MarketOperations::ClaimBorrowerRewards { to }) => {
                to_binary(&query_market_claim_borrower_rewards_msgs(deps, env, to)?)
            }
            Ok(MarketOperations::ClaimLenderRewards { to }) => {
                to_binary(&query_market_claim_lender_rewards_msgs(deps, env, to)?)
            }

            _ => match from_binary(&msg) {
                Ok(CustodyOperations::ClaimRewards {
                    collateral,
                    recipient,
                }) => to_binary(&query_custody_claim_rewards_msgs(
                    deps, env, collateral, recipient,
                )?),
                _ => match from_binary(&msg) {
                    Ok(LiquidationOperations::SubmitBid {
                        sender,
                        amount,
                        collateral_token,
                        premium_slot,
                    }) => to_binary(&query_liquidation_submit_bid_msg(
                        deps,
                        env,
                        sender,
                        amount,
                        collateral_token,
                        premium_slot,
                    )?),
                    Ok(LiquidationOperations::RetractBid { bid_idx, amount }) => to_binary(
                        &query_liquidation_retract_bid_msg(deps, env, bid_idx, amount)?,
                    ),
                    Ok(LiquidationOperations::ActivateBids {
                        collateral_token,
                        bids_idx,
                    }) => to_binary(&query_liquidation_activate_bids_msg(
                        deps,
                        env,
                        collateral_token,
                        bids_idx,
                    )?),
                    Ok(LiquidationOperations::ClaimLiquidations {
                        collateral_token,
                        bids_idx,
                    }) => to_binary(&query_liquidation_claim_liquidations_msg(
                        deps,
                        env,
                        collateral_token,
                        bids_idx,
                    )?),
                    Ok(LiquidationOperations::ClaimLendingRewards {
                        collateral_token,
                        bids_idx,
                    }) => to_binary(&query_liquidation_claim_lending_rewards_msg(
                        deps,
                        env,
                        collateral_token,
                        bids_idx,
                    )?),

                    _ => match from_binary(&msg) {
                        Ok(OverseerOperations::ProvideAndLockCollateral {
                            executor_addr,
                            sender,
                            collateral,
                            amount,
                        }) => to_binary(&query_overseer_provide_and_lock_collateral_msg(
                            deps,
                            env,
                            executor_addr,
                            sender,
                            collateral,
                            amount,
                        )?),
                        Ok(OverseerOperations::UnlockAndWithdrawCollateral {
                            sender,
                            collateral,
                            amount,
                        }) => to_binary(&query_overseer_unlock_and_withdraw_collateral_msg(
                            deps, env, sender, collateral, amount,
                        )?),

                        _ => Err(cosmwasm_std::StdError::NotFound {
                            kind: "Operations not found".to_string(),
                        }),
                    },
                },
            },
        },
    }
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?,
        overseer: deps.api.addr_humanize(&config.overseer)?,
        market: deps.api.addr_humanize(&config.market)?,
        liquidation: deps.api.addr_humanize(&config.liquidation)?,
        stable_addr: deps.api.addr_humanize(&config.stable_addr)?,
        a_stable_contract: deps.api.addr_humanize(&config.a_stable_contract)?,
        a_stable_contract_reward: deps.api.addr_humanize(&config.a_stable_contract_reward)?,
    })
}

pub fn query_collateral_info(
    deps: Deps,
    _env: Env,
    collateral: Addr,
) -> StdResult<CollateralInfoResponse> {
    let collateral_info = read_collateral_info(
        deps.storage,
        &deps.api.addr_canonicalize(collateral.as_str())?,
    )?;

    Ok(CollateralInfoResponse {
        collateral: deps.api.addr_humanize(&collateral_info.collateral).unwrap(),
        custody_contract: deps
            .api
            .addr_humanize(&collateral_info.custody_contract)
            .unwrap(),
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
