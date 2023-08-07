use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg, Uint128};
use orchai::orai_staking::{BondType, UnBondType};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    // orai staking
    pub orai_staking_hub: Addr,
    pub orai_staking_reward: Addr,
    pub orai_staking_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        // orai staking
        orai_staking_hub: Option<Addr>,
        orai_staking_reward: Option<Addr>,
        orai_staking_denom: Option<String>,
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

    // staking
    pub orai_staking_hub: Addr,
    pub orai_staking_reward: Addr,
    pub orai_staking_denom: String,
}

#[cw_serde]
pub enum StakingOperations {
    Bond {
        sender: Option<Addr>,
        bond_type: BondType,
        amount: Option<Uint128>,
    },
    WithdrawUnbonded {
        unbond_type: UnBondType,
    },
    Unbond {
        executor_addr: Addr,
        sender: Addr,
        token: Addr,
        amount: Option<Uint128>,
    },
    /// return the accrued reward in uusd to the user.
    ClaimRewards {
        recipient: Option<String>,
    },
    Convert {
        executor_addr: Addr,
        sender: Addr,
        from_token: Addr,
        amount: Option<Uint128>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}
