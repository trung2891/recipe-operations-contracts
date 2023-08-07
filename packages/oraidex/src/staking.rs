use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

use crate::asset::{Asset, AssetInfo};

#[cw_serde]
pub enum StakingExecuteMsg {
    Bond {
        asset_info: AssetInfo,
    },
    Unbond {
        asset_info: AssetInfo,
        amount: Uint128,
    },
    Withdraw {
        // If the asset token is not given, then all rewards are withdrawn
        asset_info: Option<AssetInfo>,
    },
}

#[cw_serde]
pub enum StakingQueryMsg {
    RewardInfo {
        staker_addr: Addr,
        asset_info: Option<AssetInfo>,
    },
}

#[cw_serde]
pub struct RewardInfoResponse {
    pub staker_addr: Addr,
    pub reward_infos: Vec<RewardInfoResponseItem>,
}

#[cw_serde]
pub struct RewardInfoResponseItem {
    pub asset_info: AssetInfo,
    pub bond_amount: Uint128,
    pub pending_reward: Uint128,
    pub pending_withdraw: Vec<Asset>,
    // returns true if the position should be closed to keep receiving rewards
    // with the new lp token
    pub should_migrate: Option<bool>,
}
