use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub strategy_contract: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        strategy_contract: Option<Addr>,
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
    pub strategy_contract: Addr,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum StrategyOperations {
    VerifyStrategy { creator: Addr, id: String },
}
