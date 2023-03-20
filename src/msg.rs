use crate::assets::Asset;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub asset_infos: Vec<crate::assets::AssetInfo>,
    pub strategy_infos: Vec<Addr>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit { assets: Vec<Asset> },
    Bond {},
    Unbond { id: String },
    StartUnbond { id: String, amount: Uint128 },
    Callback { action: CallbackMsg },
}

#[cw_serde]
pub enum CallbackMsg {
    BondResponse {
        share_amount: Uint128,
        bond_id: String,
    },
    StartUnbondResponse {
        share_amount: Uint128,
        unbond_id: String,
    },
    UnbondResponse {
        share_amount: Uint128,
        unbond_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(ConfigResponse)]
    UserInfo { user: String },
    #[returns(BondInfoResponse)]
    BondInfo { id: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}
#[cw_serde]
pub struct UserResponse {
    pub shares: Uint128,
    pub bonded: Uint128,
    pub strategies: Vec<String>,
}

#[cw_serde]
pub struct BondInfoResponse {
    pub addr: Addr,
    pub amount: Uint128,
    pub bonded: bool,
    pub unbonded: Uint128,
}
