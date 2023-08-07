use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage};
use cosmwasm_storage::{Bucket, ReadonlyBucket};
use cw_storage_plus::Item;

// put the length bytes at the first for compatibility with legacy singleton store
pub const CONFIG: Item<Config> = Item::new("\u{0}\u{6}config");

static PREFIX_COLLATERAL_INFO: &[u8] = b"col_info";
#[cw_serde]
pub struct Config {
    pub owner: CanonicalAddr,
    pub overseer: CanonicalAddr,
    pub market: CanonicalAddr,
    pub liquidation: CanonicalAddr,
    pub stable_addr: CanonicalAddr,
    pub a_stable_contract: CanonicalAddr,
    pub a_stable_contract_reward: CanonicalAddr,
}

#[cw_serde]
pub struct CollateralInfo {
    pub collateral: CanonicalAddr,
    pub custody_contract: CanonicalAddr,
}

pub fn store_collateral_info(
    storage: &mut dyn Storage,
    collateral_token: &CanonicalAddr,
    collateral_info: &CollateralInfo,
) -> StdResult<()> {
    let mut collateral_info_bucket: Bucket<CollateralInfo> =
        Bucket::new(storage, PREFIX_COLLATERAL_INFO);
    collateral_info_bucket.save(collateral_token.as_slice(), collateral_info)?;
    Ok(())
}

pub fn read_collateral_info(
    storage: &dyn Storage,
    collateral_token: &CanonicalAddr,
) -> StdResult<CollateralInfo> {
    let collateral_info_bucket: ReadonlyBucket<CollateralInfo> =
        ReadonlyBucket::new(storage, PREFIX_COLLATERAL_INFO);
    collateral_info_bucket
        .load(collateral_token.as_slice())
        .map_err(|_| StdError::generic_err("Collateral is not whitelisted"))
}
