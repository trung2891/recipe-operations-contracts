use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

use crate::asset::AssetInfo;

#[cw_serde]
pub enum SwapOperation {
    // swap cw20 token
    OraiSwap {
        offer_asset_info: AssetInfo,
        ask_asset_info: AssetInfo,
    },
}

impl SwapOperation {
    pub fn get_target_asset_info(&self) -> AssetInfo {
        match self {
            SwapOperation::OraiSwap { ask_asset_info, .. } => ask_asset_info.clone(),
        }
    }

    pub fn get_offer_asset_info(&self) -> AssetInfo {
        match self {
            SwapOperation::OraiSwap {
                offer_asset_info, ..
            } => offer_asset_info.clone(),
        }
    }
}

#[cw_serde]
pub enum OraiswapExecuteMsg {
    /// Execute multiple BuyOperation
    ExecuteSwapOperations {
        operations: Vec<SwapOperation>,
        minimum_receive: Option<Uint128>,
        to: Option<Addr>,
    },

    WithdrawLiquidity {},
}
