use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint256;

#[cw_serde]
pub enum MarketExecuteMsg {
    BorrowStable {
        borrow_amount: Uint256,
        to: Option<String>,
    },
    ClaimRewards {
        to: Option<String>,
    },
    /// Return stable coins to a user
    /// according to exchange rate
    RedeemStable {},
    DepositStable {},
    RepayStable {},
    RepayStableFor {
        borrower: String,
    },
}

#[cw_serde]
pub enum AStableRewardExecuteMsg {
    ClaimRewards { recipient: Option<String> },
}
