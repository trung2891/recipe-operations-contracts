#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
// use cw2::set_contract_version;

use crate::error::ContractError;

use crate::msg::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, StakingOperations,
};
use crate::orai_staking_operations::{
    query_orai_staking_bond_msg, query_orai_staking_claim_rewards_msg,
    query_orai_staking_convert_msg, query_orai_staking_unbond_msg,
};

use crate::state::{Config, CONFIG};

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

            orai_staking_hub: deps.api.addr_canonicalize(msg.orai_staking_hub.as_str())?,
            orai_staking_reward: deps
                .api
                .addr_canonicalize(msg.orai_staking_reward.as_str())?,
            orai_staking_denom: msg.orai_staking_denom,
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
            orai_staking_hub,
            orai_staking_reward,
            orai_staking_denom,
        } => execute_update_config(
            deps,
            env,
            info,
            owner,
            orai_staking_hub,
            orai_staking_reward,
            orai_staking_denom,
        ),
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<Addr>,
    orai_staking_hub: Option<Addr>,
    orai_staking_reward: Option<Addr>,
    orai_staking_denom: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if deps.api.addr_humanize(&config.owner)? != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }

    if let Some(orai_staking_hub) = orai_staking_hub {
        config.orai_staking_hub = deps.api.addr_canonicalize(orai_staking_hub.as_str())?;
    }
    if let Some(orai_staking_reward) = orai_staking_reward {
        config.orai_staking_reward = deps.api.addr_canonicalize(orai_staking_reward.as_str())?;
    }
    if let Some(orai_staking_denom) = orai_staking_denom {
        config.orai_staking_denom = orai_staking_denom;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps, env)?),

        QueryMsg::Messages { msg } => match from_binary(&msg) {
            Ok(StakingOperations::Bond {
                sender,
                bond_type,
                amount,
            }) => to_binary(&query_orai_staking_bond_msg(
                deps, env, sender, bond_type, amount,
            )?),
            Ok(StakingOperations::Unbond {
                executor_addr,
                sender,
                token,
                amount,
            }) => to_binary(&query_orai_staking_unbond_msg(
                deps,
                env,
                executor_addr,
                sender,
                token,
                amount,
            )?),
            Ok(StakingOperations::ClaimRewards { recipient }) => {
                to_binary(&query_orai_staking_claim_rewards_msg(deps, env, recipient)?)
            }
            Ok(StakingOperations::Convert {
                executor_addr,
                sender,
                from_token,
                amount,
            }) => to_binary(&query_orai_staking_convert_msg(
                deps,
                env,
                executor_addr,
                sender,
                from_token,
                amount,
            )?),
            _ => Err(cosmwasm_std::StdError::NotFound {
                kind: "Operations not found".to_string(),
            }),
        },
    }
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?,
        orai_staking_hub: deps.api.addr_humanize(&config.orai_staking_hub)?,
        orai_staking_reward: deps.api.addr_humanize(&config.orai_staking_reward)?,
        orai_staking_denom: config.orai_staking_denom,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
