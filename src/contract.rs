use crate::assets::{Asset, CoinsExt, VaultInfo};
use crate::error::ContractError;
use crate::helpers::ExecuteMsg as StrategyExecuteMsg;
use crate::helpers::{self, check_authorization};
use crate::msg::{
    BondInfoResponse, CallbackMsg, ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg,
    UserResponse,
};
use crate::state::{
    BondConuter, BondInfo, Config, UserInfo, CONFIG, COUNTER, STRATEGIES, USER_INFO,
};
use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;

// Version info, for migration info
const CONTRACT_NAME: &str = "bond";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if msg.asset_infos.len() != 2 {
        return Err(StdError::generic_err("asset_infos must contain exactly two elements").into());
    }

    if msg.asset_infos[0] == msg.asset_infos[1] {
        return Err(ContractError::DoublingAssets {});
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        vault_info: VaultInfo {
            contract_addr: env.contract.address,
            asset_infos: msg.asset_infos.clone(),
            stratgy_infos: msg.strategy_infos.clone(),
        },
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { assets } => deposit(deps, env, info, assets),
        ExecuteMsg::Bond {} => bond(deps, env, info),
        ExecuteMsg::StartUnbond { id, amount } => start_unbond(deps, env, info, id, amount),
        ExecuteMsg::Unbond { id } => unbond(deps, env, info, id),
        ExecuteMsg::Callback { action } => handle_callback(deps, env, info, action),
        
    }
}

pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    assets: Vec<Asset>,
) -> Result<Response, ContractError> {
    if assets.len() != 2 {
        return Err(StdError::generic_err("asset_infos must contain exactly two elements").into());
    }

    let config = CONFIG.load(deps.storage)?;
    info.funds
        .assert_coins_properly_sent(&assets, &config.vault_info.asset_infos)?;
    info.funds.assert_coin_ratio()?;

    let shares = info.funds.calc_shares();

    USER_INFO.save(deps.storage, &info.sender, &UserInfo {
        shares: shares,
        bonded: 0u128.into(),
        strategies: [].into(),
    })?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "deposit"),
        attr("sender", info.sender),
        attr("shares", shares),
    ]))
}

pub fn bond(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let user_info = USER_INFO.load(deps.storage, &info.sender)?;

    //chekc does user has enought shares to be bonded
    if user_info.bonded > user_info.shares {
        return Err(ContractError::InsufficientBalance {});
    };
    //calculate how much user can bond
    let shares_to_bond = (user_info.shares.checked_sub(user_info.bonded)).unwrap();
    let strategies = config.vault_info.stratgy_infos;
    USER_INFO.update(
        deps.storage,
        &info.sender,
        |user_info: Option<UserInfo>| -> StdResult<_> {
            let mut user_info = user_info.unwrap();
            user_info.bonded = user_info.bonded + shares_to_bond;
            Ok(user_info)
        },
    )?;

    let shares_to_bond = shares_to_bond.checked_div(2u128.into()).unwrap();

    let mut response = Response::new();
    response = response.add_attributes(vec![attr("action", "bond")]);

    for strategy in strategies {
        let strategy_response: helpers::ConfigResponse = deps
            .querier
            .query_wasm_smart(strategy.clone(), &helpers::QueryMsg::Config {})
            .unwrap();
        let bond_id_count = COUNTER
            .load(deps.storage)
            .unwrap_or(BondConuter { count: 1 });
        let bond_id = bond_id_count.count.to_string();
        response = response.add_attributes(vec![attr("strategy", bond_id.clone())]);
        response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: strategy.clone().to_string(),
            msg: to_binary(&StrategyExecuteMsg::Bond {
                id: bond_id.clone(),
            })?,
            funds: vec![Coin {
                denom: strategy_response.config.local_denom.clone(),
                amount: shares_to_bond,
            }],
        }));

        //update user strategies
        USER_INFO.update(
            deps.storage,
            &info.sender,
            |user_info: Option<UserInfo>| -> StdResult<_> {
                let mut user_info = user_info.unwrap();
                user_info.strategies.push(bond_id.clone());
                Ok(user_info)
            },
        )?;

        // Create bond info associated with bond_id
        STRATEGIES.update(
            deps.storage,
            &bond_id.to_string(),
            |bond_info: Option<BondInfo>| -> StdResult<_> {
                let bond_info = bond_info.unwrap_or(BondInfo {
                    addr: strategy.clone(),
                    owner_addr: info.sender.clone(),
                    denom: strategy_response.config.local_denom.clone(),
                    amount: shares_to_bond,
                    bonded: false,
                    unbonded_amount: Uint128::new(0u128),
                    unbonded: false,
                    unbond_time: strategy_response.config.lock_period,
                });
                Ok(bond_info)
            },
        )?;

        // increment the strategies count
        COUNTER.save(
            deps.storage,
            &BondConuter {
                count: bond_id_count.count.checked_add(1).unwrap(),
            },
        )?;
    }
    Ok(response)
}

pub fn start_unbond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let user_info = USER_INFO.load(deps.storage, &info.sender)?;

    if amount.u128() == 0u128 {
        return Err(ContractError::BadAmount {});
    }

    check_authorization(user_info.strategies, id.clone())?;

    let strategy = STRATEGIES.load(deps.storage, &id)?;
    //check does user can start unbond of this
    if !strategy.bonded {
        return Err(ContractError::Notbonded {});
    }
    if strategy.unbonded_amount > 0u128.into() {
        return Err(ContractError::Unbonded {});
    }
    if amount > strategy.amount {
        return Err(ContractError::BadAmount {});
    }

    let response = Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: strategy.addr.to_string(),
        msg: to_binary(&StrategyExecuteMsg::StartUnbond {
            id: id.clone(),
            share_amount: amount,
        })?,
        funds: vec![],
    }));
    // update the strategy
    STRATEGIES.update(
        deps.storage,
        &id,
        |bond_info: Option<BondInfo>| -> StdResult<_> {
            let mut bond_info = bond_info.unwrap();
            bond_info.unbonded_amount = amount;
            bond_info.unbonded = true;
            Ok(bond_info)
        },
    )?;

    Ok(response.add_attributes(vec![
        attr("action", "start_unbond"),
        attr("strategy", id),
        attr("amount", amount),
    ]))
}

pub fn unbond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
) -> Result<Response, ContractError> {
    let user_info = USER_INFO.load(deps.storage, &info.sender)?;
    let bond_id = id.clone();

    //Check is user authorized to unbond the strategy
    check_authorization(user_info.strategies.clone(), bond_id.clone())?;

    let strategy = STRATEGIES.load(deps.storage, &bond_id)?;
    let amount = strategy.unbonded_amount;

    if _env.block.time.seconds() < strategy.unbond_time {
        return Err(ContractError::BondTime {});
    }

    if strategy.unbonded {
        return Err(ContractError::Unbonded {});
    }

    if strategy.amount == amount {
        //remove user strategy
        let mut updated_strategies = user_info.strategies;
        if let Some(index) = updated_strategies.iter().position(|value| *value == id) {
            updated_strategies.swap_remove(index);
        }

        USER_INFO.update(
            deps.storage,
            &info.sender,
            |user_info: Option<UserInfo>| -> StdResult<_> {
                let mut user_info = user_info.unwrap();
                user_info.shares = user_info.shares - strategy.amount;
                user_info.bonded = user_info.bonded - strategy.amount;
                user_info.strategies = updated_strategies;
                Ok(user_info)
            },
        )?;
    } else {
        USER_INFO.update(
            deps.storage,
            &info.sender,
            |user_info: Option<UserInfo>| -> StdResult<_> {
                let mut user_info = user_info.unwrap();
                user_info.shares = user_info.shares - amount;
                user_info.bonded = user_info.bonded - amount;
                Ok(user_info)
            },
        )?;
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "unbond"),
        attr("sender", info.sender),
        attr("strategy", bond_id),
        attr("amount", amount),
    ]))
}

pub fn handle_callback(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: CallbackMsg,
) -> Result<Response, ContractError> {
    match msg {
        CallbackMsg::BondResponse {
            share_amount,
            bond_id,
        } => handle_bond_response(deps, env, share_amount, bond_id),
        CallbackMsg::StartUnbondResponse {
             share_amount, unbond_id } => {
            handle_start_unbond_response(deps, env, share_amount, unbond_id)
        }
        CallbackMsg::UnbondResponse {
            share_amount,
            unbond_id } => handle_unbond_response(deps, env, share_amount, unbond_id),
    }
}

pub fn handle_bond_response(
    deps: DepsMut,
    _env: Env,
    share_amount: Uint128,
    bond_id: String,
) -> Result<Response, ContractError> {
    STRATEGIES.update(
        deps.storage,
        &bond_id,
        |bond_info: Option<BondInfo>| -> StdResult<_> {
            let mut bond_info = bond_info.unwrap();
            bond_info.amount = share_amount;
            bond_info.bonded = true;
            Ok(bond_info)
        },
    )?;

    Ok(
        Response::new()
            .add_attributes(vec![attr("action", "bonded"), attr("shares", share_amount)]),
    )
}

pub fn handle_start_unbond_response(
    deps: DepsMut,
    _env: Env,
    share_amount: Uint128,
    unbond_id: String,
) -> Result<Response, ContractError> {
    let block_time = _env.block.time.seconds();
    STRATEGIES.update(
        deps.storage,
        &unbond_id,
        |bond_info: Option<BondInfo>| -> StdResult<_> {
            let mut bond_info = bond_info.unwrap();
            bond_info.unbond_time = bond_info.unbond_time + block_time;
            bond_info.unbonded_amount = share_amount;
            bond_info.unbonded = false;
            Ok(bond_info)
        },
    )?;

    Ok(
        Response::new()
            .add_attributes(vec![attr("action", "unbond"), attr("unbond_id", unbond_id), attr("share amount", share_amount)]),
    )
}

pub fn handle_unbond_response(
    deps: DepsMut,
    _env: Env,
    share_amount: Uint128,
    bond_id: String,
) -> Result<Response, ContractError> {
    let strategy_info = STRATEGIES.load(deps.storage, &bond_id)?;
    let response = Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: strategy_info.owner_addr.to_string(),
        amount: vec![Coin {
            denom: strategy_info.denom,
            amount: strategy_info.unbonded_amount,
        }],
    }));


        let strategy_response: helpers::ConfigResponse = deps
        .querier
        .query_wasm_smart(strategy_info.addr.clone(), &helpers::QueryMsg::Config {})
        .unwrap();
    STRATEGIES.update(
        deps.storage,
        &bond_id,
        |bond_info: Option<BondInfo>| -> StdResult<_> {
            let mut bond_info = bond_info.unwrap();
            bond_info.amount = bond_info.amount - bond_info.unbonded_amount;
            bond_info.unbonded_amount = 0u128.into();
            bond_info.unbond_time = strategy_response.config.lock_period;
            Ok(bond_info)
        },
    )?;


    Ok(response.add_attributes(vec![
        attr("action", "unbonded"),
        attr("unbond_id", bond_id),
        attr("amount", strategy_info.unbonded_amount),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::UserInfo { user } => {
            let user_addr = deps.api.addr_validate(&user)?;
            to_binary(&query_user(deps, user_addr)?)
        }
        QueryMsg::BondInfo { id } => to_binary(&query_bond_info(deps, id)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let addr = config.owner;
    Ok(ConfigResponse { owner: addr })
}

fn query_user(deps: Deps, user_addr: Addr) -> StdResult<UserResponse> {
    let user_info = USER_INFO.load(deps.storage, &user_addr)?;
    let shares = user_info.shares;
    let bonded = user_info.bonded;
    let strategies = user_info.strategies;
    Ok(UserResponse {
        shares,
        bonded,
        strategies,
    })
}

fn query_bond_info(deps: Deps, id: String) -> StdResult<BondInfoResponse> {
    let bond_info = STRATEGIES.load(deps.storage, &id)?;
    let addr = bond_info.addr;
    let amount = bond_info.amount;
    let bonded = bond_info.bonded;
    let unbonded = bond_info.unbonded_amount;
    Ok(BondInfoResponse {
        addr,
        amount,
        bonded,
        unbonded,
    })
}

