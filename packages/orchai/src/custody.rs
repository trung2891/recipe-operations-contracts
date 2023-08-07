use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint256;

#[cw_serde]
pub enum CustodyExecuteMsg {
    /// Deposit collateral token
    DepositCollateral {},
    /// Withdraw spendable collateral token.
    /// If the amount is not given,
    /// return all spendable collateral
    WithdrawCollateral { amount: Option<Uint256> },

    /// return the accrued reward in uusd to the user.
    ClaimRewards { recipient: Option<String> },
}

#[cw_serde]
pub enum CustodyQueryMsg {
    Borrower { address: String },
}

#[cw_serde]
pub struct BorrowerResponse {
    pub borrower: String,
    pub balance: Uint256,
    pub spendable: Uint256,
}
