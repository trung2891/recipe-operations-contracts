#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    chain::{query_delegate_msg, query_undelegate_msg},
    error::ContractError,
    msg::{
        ChainOperations, ConfigResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg,
        TokenOperations, WrappedOperations,
    },
    state::{Config, CONFIG},
    tokens::{
        query_token_send_from_msg, query_token_send_msg, query_token_transfer_from_msg,
        query_token_transfer_msg,
    },
    wrapped::{query_wrapped_convert_to_cw20, query_wrapped_convert_to_denom},
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
            wrapped_contract: deps.api.addr_canonicalize(msg.wrapped_contract.as_str())?,
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
            wrapped_contract,
        } => execute_update_config(deps, info, owner, wrapped_contract),
    }
}

pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    owner: Option<Addr>,
    wrapped_contract: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if deps.api.addr_humanize(&config.owner)? != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
    }

    if let Some(wrapped_contract) = wrapped_contract {
        config.wrapped_contract = deps.api.addr_canonicalize(wrapped_contract.as_str())?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Messages { msg } => match from_binary(&msg) {
            Ok(TokenOperations::TokenTransfer {
                token,
                sender,
                recipient,
                amount,
            }) => to_binary(&query_token_transfer_msg(
                deps, token, sender, recipient, amount,
            )?),
            Ok(TokenOperations::TokenTransferFrom {
                token,
                owner,
                recipient,
                amount,
            }) => to_binary(&query_token_transfer_from_msg(
                deps, token, owner, recipient, amount,
            )?),
            Ok(TokenOperations::TokenSend {
                token,
                sender,
                contract,
                amount,
                msg,
            }) => to_binary(&query_token_send_msg(
                deps, token, sender, contract, amount, msg,
            )?),
            Ok(TokenOperations::TokenSendFrom {
                token,
                owner,
                contract,
                amount,
                msg,
            }) => to_binary(&query_token_send_from_msg(
                deps, token, owner, contract, amount, msg,
            )?),
            _ => match from_binary(&msg) {
                Ok(WrappedOperations::ConvertToCw20 {
                    from_denom,
                    executor_addr,
                    amount,
                }) => to_binary(&query_wrapped_convert_to_cw20(
                    deps,
                    from_denom,
                    executor_addr,
                    amount,
                )?),
                Ok(WrappedOperations::ConvertToDenom {
                    from_token,
                    target_denom,
                    executor_addr,
                    sender,
                    amount,
                }) => to_binary(&query_wrapped_convert_to_denom(
                    deps,
                    from_token,
                    target_denom,
                    executor_addr,
                    sender,
                    amount,
                )?),
                _ => match from_binary(&msg) {
                    Ok(ChainOperations::ChainDelegate {
                        executor_addr,
                        validator,
                        denom,
                        amount,
                    }) => to_binary(&query_delegate_msg(
                        deps,
                        executor_addr,
                        validator,
                        denom,
                        amount,
                    )?),
                    Ok(ChainOperations::ChainUndelegate {
                        executor_addr,
                        validator,
                        denom,
                        amount,
                    }) => to_binary(&query_undelegate_msg(
                        deps,
                        executor_addr,
                        validator,
                        denom,
                        amount,
                    )?),
                    _ => Err(cosmwasm_std::StdError::NotFound {
                        kind: "Operations not found".to_string(),
                    }),
                },
            },
        },
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
