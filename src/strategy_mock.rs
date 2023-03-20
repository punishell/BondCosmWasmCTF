use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Binary, Empty, Response, StdResult, Uint128};
use cw_multi_test::{Contract, ContractWrapper};
use cw_storage_plus::Map;
use lazy_static::lazy_static;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

use crate::helpers::{Config, ConfigResponse};

// This lazy static use allows you the dev to set the aust token addr before you use the anchor mock so that you can mock out AUST as needed.
lazy_static! {
    // This lazily made static uses a ReadWrite lock to ensure some form of safety on setting/getting values and means you dont need to wrap the code in an unsafe block which looks icky
    static ref TOKEN_ADDR: RwLock<String> = RwLock::new("string".to_string());
}

// Simple mocked instantiate with no params so devs can use it easily
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MockInstantiateMsg {}

// Mocked ExecuteMsg with some CW20 related functions, maybe these are needed at all but it gives you a bigger mock to play with.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MockExecuteMsg {
    Bond {},
    StartUnbond {},
    Unbond {},
}

// We define a custom struct for each query response
#[cw_serde]
pub enum MockQueryMsg {
    Config {},
}

pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub fn contract_strategy_mock1() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        |_deps, _, _info, msg: MockExecuteMsg| -> StdResult<Response> {
            match msg {
                MockExecuteMsg::Bond {} => Ok(Response::new()),
                MockExecuteMsg::StartUnbond {} => Ok(Response::new()),
                MockExecuteMsg::Unbond {} => Ok(Response::new()),
            }
        },
        |_, _, _, _: MockInstantiateMsg| -> StdResult<Response> { Ok(Response::default()) },
        |_, _, msg: MockQueryMsg| -> StdResult<Binary> {
            match msg {
                MockQueryMsg::Config {} => Ok(to_binary(&mock_config_response())?),
            }
        },
    );
    Box::new(contract)
}

//
// Mocked funcs to return data
//

pub fn mock_config_response() -> ConfigResponse {
    ConfigResponse {
        config: Config {
            lock_period: 100u64,
            local_denom: "token1".to_string(),
        },
    }
    
}

pub fn contract_strategy_mock2() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        |_deps, _, _info, msg: MockExecuteMsg| -> StdResult<Response> {
            match msg {
                MockExecuteMsg::Bond {} => Ok(Response::new()),
                MockExecuteMsg::StartUnbond {} => Ok(Response::new()),
                MockExecuteMsg::Unbond {} => Ok(Response::new()),
            }
        },
        |_, _, _, _: MockInstantiateMsg| -> StdResult<Response> { Ok(Response::default()) },
        |_, _, msg: MockQueryMsg| -> StdResult<Binary> {
            match msg {
                MockQueryMsg::Config {} => Ok(to_binary(&mock_config_response2())?),
            }
        },
    );
    Box::new(contract)
}

pub fn mock_config_response2() -> ConfigResponse {
 ConfigResponse {
        config: Config {
            lock_period: 100u64,
            local_denom: "token2".to_string(),
        },
    }
    
}
