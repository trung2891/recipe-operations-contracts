use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub wrapped_contract: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        wrapped_contract: Option<Addr>,
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
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum TokenOperations {
    TokenTransfer {
        token: Addr,
        sender: Option<Addr>,
        recipient: Addr,
        amount: Option<Uint128>,
    },
    TokenTransferFrom {
        token: Addr,
        owner: Addr,
        recipient: Addr,
        amount: Option<Uint128>,
    },
    TokenSend {
        token: Addr,
        sender: Option<Addr>,
        contract: Addr,
        amount: Option<Uint128>,
        msg: Binary,
    },
    TokenSendFrom {
        token: Addr,
        owner: Addr,
        contract: Addr,
        amount: Option<Uint128>,
        msg: Binary,
    },
}

#[cw_serde]
pub enum WrappedOperations {
    ConvertToDenom {
        from_token: Addr,
        target_denom: String,
        executor_addr: Addr,
        sender: Addr,
        amount: Option<Uint128>,
    },
    ConvertToCw20 {
        from_denom: String,
        executor_addr: Addr,
        amount: Option<Uint128>,
    },
}

#[cw_serde]
pub enum ChainOperations {
    ChainDelegate {
        executor_addr: Addr,
        validator: String,
        denom: String,
        amount: Option<Uint128>,
    },
    ChainUndelegate {
        executor_addr: Addr,
        validator: String,
        denom: String,
        amount: Option<Uint128>,
    },
}
