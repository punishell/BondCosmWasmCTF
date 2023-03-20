use crate::assets::VaultInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub vault_info: VaultInfo,
}

#[cw_serde]
pub struct UserInfo {
    pub shares: Uint128,
    pub bonded: Uint128,
    pub strategies: Vec<String>,
}

#[cw_serde]
pub struct BondInfo {
    pub addr: Addr,
    pub owner_addr: Addr,
    pub denom: String,
    pub amount: Uint128,
    pub bonded: bool,
    pub unbonded_amount: Uint128,
    pub unbonded: bool,
    pub unbond_time: u64,
}

#[cw_serde]
pub struct BondConuter {
    pub count: u64,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const USER_INFO_KEY: &str = "user_info";
pub const USER_INFO: Map<&Addr, UserInfo> = Map::new(USER_INFO_KEY);

pub const STRATEGIES_KEY: &str = "strategies";
pub const STRATEGIES: Map<&str, BondInfo> = Map::new(STRATEGIES_KEY);

pub const COUNTER_KEY: &str = "counter";
pub const COUNTER: Item<BondConuter> = Item::new(COUNTER_KEY);
