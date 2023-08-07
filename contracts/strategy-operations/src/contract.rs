#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError,
    msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, StrategyOperations},
    state::{Config, CONFIG},
    strategy::query_verify_strategy_msg,
};

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
            strategy_contract: deps.api.addr_canonicalize(msg.strategy_contract.as_str())?,
        },
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            strategy_contract,
        } => execute_update_config(deps, info, owner, strategy_contract),
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<Addr>,
    strategy_contract: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if deps.api.addr_humanize(&config.owner)? != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }

    if let Some(strategy_contract) = strategy_contract {
        config.strategy_contract = deps.api.addr_canonicalize(strategy_contract.as_str())?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Messages { msg } => match from_binary(&msg) {
            Ok(StrategyOperations::VerifyStrategy { creator, id }) => {
                to_binary(&query_verify_strategy_msg(deps, creator, id)?)
            }
            _ => Err(cosmwasm_std::StdError::NotFound {
                kind: "Operations not found".to_string(),
            }),
        },
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?,
        strategy_contract: deps.api.addr_humanize(&config.strategy_contract)?,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
