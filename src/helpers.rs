use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub enum QueryMsg {
    Config {},
}

#[cw_serde]
pub enum ExecuteMsg {
    Bond { id: String },
    StartUnbond { id: String, share_amount: Uint128 },
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct Config {
    // The lock period of the strategy
    pub lock_period: u64,
    // the denom on the wasm chain
    pub local_denom: String,
}

pub fn check_authorization(strategies: Vec<String>, id: String) -> Result<(), ContractError> {
    if strategies.contains(&id) {
        Ok(())
    } else {
        Err(ContractError::Unauthorized {})
    }
}
