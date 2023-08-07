use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

use crate::asset::Asset;

#[cw_serde]
pub enum PairExecuteMsg {
    ProvideLiquidity {
        assets: [Asset; 2],
        slippage_tolerance: Option<Decimal>,
        receiver: Option<Addr>,
    },
    WithdrawLiquidity {},
}
