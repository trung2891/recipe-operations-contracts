use cosmwasm_std::{Addr, QuerierWrapper, StdResult, Uint128};
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20QueryMsg};

pub fn query_token_balance(
    querier: &QuerierWrapper,
    contract_addr: Addr,
    account_addr: Addr,
) -> StdResult<Uint128> {
    let res: Cw20BalanceResponse = querier.query_wasm_smart(
        contract_addr,
        &Cw20QueryMsg::Balance {
            address: account_addr.to_string(),
        },
    )?;

    // load balance form the token contract
    Ok(res.balance)
}
