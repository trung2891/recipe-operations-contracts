#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError,
    ica::{
        query_delegate_on_host_chain, query_transfer_from_host_chain_msg,
        query_transfer_to_host_chain_msg,
    },
    msg::{ConfigResponse, ExecuteMsg, IcaOperations, InstantiateMsg, MigrateMsg, QueryMsg},
    state::{Config, CONFIG},
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
            chain_id: msg.chain_id,
            ica_connection_id: msg.ica_connection_id,
            timeout_default: msg.timeout_default,
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
            timeout_default,
        } => execute_update_config(deps, info, owner, timeout_default),
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<Addr>,
    timeout_default: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if deps.api.addr_humanize(&config.owner)? != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }

    if let Some(timeout_default) = timeout_default {
        config.timeout_default = timeout_default;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Messages { msg } => match from_binary(&msg) {
            Ok(IcaOperations::TransferToHostChain {
                executor_addr,
                denom,
                amount,
                receiver,
                source_port,
                source_channel,
            }) => to_binary(&query_transfer_to_host_chain_msg(
                deps,
                env,
                executor_addr,
                denom,
                amount,
                receiver,
                source_port,
                source_channel,
            )?),
            Ok(IcaOperations::TransferFromHostChain {
                executor_addr,
                denom,
                amount,
                sender,
                receiver,
                source_port,
                source_channel,
            }) => to_binary(&query_transfer_from_host_chain_msg(
                deps,
                env,
                executor_addr,
                denom,
                amount,
                sender,
                receiver,
                source_port,
                source_channel,
            )?),
            Ok(IcaOperations::DelegateOnHostChain {
                executor_addr,
                delegator,
                validator,
                denom,
                amount,
            }) => to_binary(&query_delegate_on_host_chain(
                deps,
                env,
                executor_addr,
                delegator,
                validator,
                denom,
                amount,
            )?),
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
        chain_id: config.chain_id,
        ica_connection_id: config.ica_connection_id,
        timeout_default: config.timeout_default,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
