use cosmos_sdk_proto::{
    cosmos::base::v1beta1::Coin as ProtoCoin,
    cosmos::staking::v1beta1::MsgDelegate,
    ibc::applications::{
        interchain_accounts::v1::{MsgRegisterAccount, MsgSubmitTx},
        transfer::v1::MsgTransfer,
    },
    traits::{MessageExt, TypeUrl},
    Any,
};
use cosmwasm_std::{Coin, CosmosMsg, Deps, Env, IbcMsg, IbcTimeout, StdResult, Timestamp, Uint128};

use crate::state::CONFIG;

pub fn query_register_interchain_account_msg(
    deps: Deps,
    executor_addr: String,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;

    let tx = MsgRegisterAccount {
        owner: executor_addr,
        connection_id: config.ica_connection_id,
        version: "".to_string(),
    };

    Ok(vec![CosmosMsg::Stargate {
        type_url: MsgRegisterAccount::TYPE_URL.to_string(),
        value: tx.to_bytes().unwrap().into(),
    }])
}

pub fn query_transfer_to_host_chain_msg(
    deps: Deps,
    env: Env,
    _executor_addr: String,
    denom: String,
    amount: Uint128,
    receiver: String,
    _source_port: String,
    source_channel: String,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;

    let ibc_msg = IbcMsg::Transfer {
        channel_id: source_channel,
        to_address: receiver,
        amount: Coin { denom, amount },
        timeout: IbcTimeout::with_timestamp(Timestamp::from_seconds(
            env.block.time.seconds() + config.timeout_default,
        )),
    };
    Ok(vec![CosmosMsg::Ibc(ibc_msg)])
}

pub fn query_transfer_from_host_chain_msg(
    deps: Deps,
    env: Env,
    executor_addr: String,
    denom: String,
    amount: Uint128,
    sender: String,
    receiver: String,
    source_port: String,
    source_channel: String,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;
    let ibc_transfer_msg = MsgTransfer {
        source_port,
        source_channel,
        token: Some(ProtoCoin {
            denom,
            amount: amount.to_string(),
        }),
        sender,
        receiver,
        timeout_height: None,
        timeout_timestamp: env.block.time.nanos() + config.timeout_default * 1000000000,
    };

    let msg = Any {
        type_url: MsgTransfer::TYPE_URL.to_string(),
        value: ibc_transfer_msg.to_bytes().unwrap().into(),
    };

    let tx = MsgSubmitTx {
        owner: executor_addr,
        connection_id: config.ica_connection_id,
        msg: Some(msg),
    };

    Ok(vec![CosmosMsg::Stargate {
        type_url: MsgSubmitTx::TYPE_URL.to_string(),
        value: tx.to_bytes().unwrap().into(),
    }])
}

pub fn query_delegate_on_host_chain(
    deps: Deps,
    _env: Env,
    executor_addr: String,
    delegator: String,
    validator: String,
    denom: String,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
    let config = CONFIG.load(deps.storage)?;

    let staking_msg = MsgDelegate {
        delegator_address: delegator,
        validator_address: validator,
        amount: Some(cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
            denom,
            amount: amount.to_string(),
        }),
    };
    let msg = Any {
        type_url: MsgDelegate::TYPE_URL.to_string(),
        value: staking_msg.to_bytes().unwrap().into(),
    };
    let tx = MsgSubmitTx {
        owner: executor_addr,
        connection_id: config.ica_connection_id,
        msg: Some(msg),
    };

    Ok(vec![CosmosMsg::Stargate {
        type_url: MsgSubmitTx::TYPE_URL.to_string(),
        value: tx.to_bytes().unwrap().into(),
    }])
}
