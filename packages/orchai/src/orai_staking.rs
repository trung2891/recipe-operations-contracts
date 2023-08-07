use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum UnBondType {
    SOrai,
    ScOrai,
}

#[cw_serde]
pub enum BondType {
    SOrai,
    ScOrai,
}

#[cw_serde]
pub enum ConvertType {
    ScOraiToSOrai,
    SOraiToScOrai,
}

#[cw_serde]
pub enum OraiStakingExecuteMsg {
    Bond {
        bond_type: BondType,
    },
    WithdrawUnbonded {
        unbond_type: UnBondType,
    },
    Unbond {},
    /// return the accrued reward in uusd to the user.
    ClaimRewards {
        recipient: Option<String>,
    },
    Convert {},
}
