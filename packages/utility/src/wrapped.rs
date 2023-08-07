use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum WrappedExecuteMsg {
    ConvertToCw20 {},
    ConvertToDenom { target_denom: String },
}
