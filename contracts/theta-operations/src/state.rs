use cosmwasm_schema::cw_serde;
use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;

// put the length bytes at the first for compatibility with legacy singleton store
pub const CONFIG: Item<Config> = Item::new("\u{0}\u{6}config");

#[cw_serde]
pub struct Config {
    pub owner: CanonicalAddr,
    pub chain_id: String,
    pub ica_connection_id: String, // connection id between ica channel
    pub timeout_default: u64, // timeout default when sending ibc transfer package between the controller and host account
}
