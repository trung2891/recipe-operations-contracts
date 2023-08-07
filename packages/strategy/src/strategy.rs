use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub enum StrategyExecuteMsg {
    VerifyStrategy { creator: Addr, id: String },
}
