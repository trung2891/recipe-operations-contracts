use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub chain_id: String,
    pub ica_connection_id: String, // connection id between ica channel
    pub timeout_default: u64, // timeout default when sending ibc transfer package between the controller and host account
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        timeout_default: Option<u64>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<CosmosMsg>)]
    Messages { msg: Binary },
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub chain_id: String,
    pub ica_connection_id: String, // connection id between ica channel
    pub timeout_default: u64, // timeout default when sending ibc transfer package between the controller and host account
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum IcaOperations {
    TransferToHostChain {
        executor_addr: String,
        denom: String,
        amount: Uint128,
        receiver: String,
        source_port: String,
        source_channel: String,
    },
    TransferFromHostChain {
        executor_addr: String,
        denom: String,
        amount: Uint128,
        sender: String,
        receiver: String,
        source_port: String,
        source_channel: String,
    },
    DelegateOnHostChain {
        executor_addr: String,
        delegator: String,
        validator: String,
        denom: String,
        amount: Uint128,
    },
    RegisterInterchainAccount {
        executor_addr: String,
    },
}
