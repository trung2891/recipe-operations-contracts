use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, CosmosMsg, Uint128, Uint256};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub overseer: Addr,
    pub market: Addr,
    pub liquidation: Addr,
    pub stable_addr: Addr,
    pub a_stable_contract: Addr,
    pub a_stable_contract_reward: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterCollateral {
        collateral: Addr,
        custody_contract: Addr,
    },
    UpdateConfig {
        owner: Option<Addr>,
        overseer: Option<Addr>,
        market: Option<Addr>,
        liquidation: Option<Addr>,
        stable_addr: Option<Addr>,
        a_stable_contract: Option<Addr>,
        a_stable_contract_reward: Option<Addr>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<CosmosMsg>)]
    Messages { msg: Binary },
    #[returns(ConfigResponse)]
    Config {},
    #[returns(CollateralInfoResponse)]
    CollateralInfo { collateral: Addr },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub overseer: Addr,
    pub market: Addr,
    pub liquidation: Addr,
    pub stable_addr: Addr,
    pub a_stable_contract: Addr,
    pub a_stable_contract_reward: Addr,
}
#[cw_serde]
pub struct CollateralInfoResponse {
    pub collateral: Addr,
    pub custody_contract: Addr,
}

#[cw_serde]
pub enum MarketOperations {
    BorrowStable {
        borrow_amount: Uint256,
        to: Option<String>,
    },
    ClaimBorrowerRewards {
        to: Option<String>,
    },
    ClaimLenderRewards {
        to: Option<String>,
    },
    RedeemStable {
        executor_addr: Addr,
        sender: Addr,
        a_stable_amount: Option<Uint128>,
    },
    DepositStable {
        executor_addr: Addr,
        sender: Addr,
        stable_amount: Option<Uint128>,
    },
    RepayStable {
        amount: Uint128,
        executor_addr: Addr,
        sender: Addr,
    },
    RepayStableFor {
        amount: Uint128,
        borrower: String,
    },
}

#[cw_serde]
pub enum CustodyOperations {
    /// return the accrued reward in uusd to the user.
    ClaimRewards {
        collateral: Addr,
        recipient: Option<String>,
    },
}

#[cw_serde]
pub enum LiquidationOperations {
    /// Withdraw a bid
    RetractBid {
        bid_idx: Uint128,
        amount: Option<Uint256>,
    },
    /// After waiting_period expires, user can activate the bid
    ActivateBids {
        collateral_token: Addr,
        bids_idx: Option<Vec<Uint128>>,
    },
    /// Claim the corresponding amount of liquidated collateral
    ClaimLiquidations {
        collateral_token: Addr,
        bids_idx: Option<Vec<Uint128>>,
    },
    ClaimLendingRewards {
        collateral_token: Addr,
        bids_idx: Option<Vec<Uint128>>,
    },
    SubmitBid {
        sender: Option<Addr>,
        amount: Option<Uint128>,
        collateral_token: Addr,
        premium_slot: u8,
    },
}

#[cw_serde]
pub enum OverseerOperations {
    ProvideAndLockCollateral {
        executor_addr: Addr,
        sender: Addr,
        collateral: Addr,
        amount: Option<Uint128>,
    },
    UnlockAndWithdrawCollateral {
        sender: Option<Addr>,
        collateral: Addr,
        amount: Option<Uint128>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}
