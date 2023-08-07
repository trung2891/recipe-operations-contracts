use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg, Decimal, Uint128};
use oraidex::{
    asset::{Asset, AssetInfo},
    router::SwapOperation,
};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub oraiswap_router: Addr,
    pub oraiswap_staking: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        oraiswap_router: Option<Addr>,
        oraiswap_staking: Option<Addr>,
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
pub enum Operations {
    SwapOperations {
        executor_addr: Addr,
        sender: Addr,
        amount: Option<Uint128>,
        operations: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<Addr>,
    },
    ProvideLiquidity {
        pair_contract: Addr,
        assets: [Asset; 2],
        slippage_tolerance: Option<Decimal>,
        receiver: Option<Addr>,
    },
    WithdrawLiquidity {
        sender: Option<Addr>,
        pair_contract: Addr,
        lp_token: Addr,
        amount: Option<Uint128>,
    },
    Bond {
        sender: Option<Addr>,
        lp_token: Addr,
        asset_info: AssetInfo,
        amount: Option<Uint128>,
    },
    Unbond {
        sender: Option<Addr>,
        asset_info: AssetInfo,
        amount: Option<Uint128>,
    },
    /// Withdraw pending rewards
    Withdraw {
        // If the asset token is not given, then all rewards are withdrawn
        asset_info: Option<AssetInfo>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub oraiswap_router: Addr,
    pub oraiswap_staking: Addr,
}

#[cw_serde]
pub struct MigrateMsg {}
