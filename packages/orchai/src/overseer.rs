use cosmwasm_schema::cw_serde;

use crate::tokens::TokensHuman;

#[cw_serde]
pub enum OverseerExecuteMsg {
    LockCollateral {
        collaterals: TokensHuman, // <(Collateral Token, Amount)>
    },
    UnlockCollateral {
        collaterals: TokensHuman, // <(Collateral Token, Amount)>
    },
}
