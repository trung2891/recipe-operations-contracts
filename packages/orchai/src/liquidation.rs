use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Uint256};

#[cw_serde]
pub enum LiquidationExecuteMsg {
    /// Withdraw a bid
    RetractBid {
        bid_idx: Uint128,
        amount: Option<Uint256>,
    },
    /// After waiting_period expires, user can activate the bid
    ActivateBids {
        collateral_token: String,
        bids_idx: Option<Vec<Uint128>>,
    },
    /// Claim the corresponding amount of liquidated collateral
    ClaimLiquidations {
        collateral_token: String,
        bids_idx: Option<Vec<Uint128>>,
    },
    ClaimLendingRewards {
        collateral_token: String,
        bids_idx: Option<Vec<Uint128>>,
    },
    SubmitBid {
        collateral_token: String,
        premium_slot: u8,
    },
}
