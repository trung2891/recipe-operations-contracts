use cosmwasm_schema::cw_serde;
use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;

// put the length bytes at the first for compatibility with legacy singleton store
pub const CONFIG: Item<Config> = Item::new("\u{0}\u{6}config");

#[cw_serde]
pub struct Config {
    pub owner: CanonicalAddr,
    pub oraiswap_router: CanonicalAddr,
    pub oraiswap_staking: CanonicalAddr,
}
