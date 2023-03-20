use cosmwasm_std::{coin, Addr, Coin, Querier, Uint128};
use cosmwasm_std::{from_binary, to_binary, BalanceResponse, BankQuery, QueryRequest};
use cw_multi_test::{App, ContractWrapper, Executor};
use cw_vault::assets::{Asset, AssetInfo};
use cw_vault::contract::{execute, instantiate, query};
use cw_vault::msg::{
    BondInfoResponse, CallbackMsg, ExecuteMsg, InstantiateMsg, QueryMsg, UserResponse,
};
use cw_vault::strategy_mock::{
    contract_strategy_mock1, contract_strategy_mock2, MockInstantiateMsg,
};

const USER1: &str = "user1";
const OWNER: &str = "owner";
const BOND_ID_1: &str = "1";
const BOND_ID_2: &str = "2";
const TOKEN_AMOUNT: u128 = 1000000u128;
const TOKEN_HALF_AMOUNT: u128 = 500000u128;

<<<<<<< HEAD
=======

>>>>>>> 0e6237bf7f992ac14d4f950abd655e412463f9d9
#[test]
fn happy_path_test() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );

    //deposit assets
    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );
    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_ok());


    //Bond Tokens
    let msg = ExecuteMsg::Bond {};
    let bond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(bond_response.is_ok());



    //mock bond callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_1.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_2.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    //mock start unbond callback for both strategies
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_AMOUNT);

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_AMOUNT);
    //update time
    update_time(&mut app, 101);

    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_1.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_ok());

    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_2.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_ok());

    //mock callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::UnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    mint_native(
        &mut app,
        contract_addr.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    let start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond_callback_respose.is_ok());

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::UnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    mint_native(
        &mut app,
        contract_addr.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );
    let start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond_callback_respose.is_ok());

    //user strategies should decrees
    let query_user_info: UserResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::UserInfo {
                user: USER1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_user_info.strategies.len(), 0);

    //user shares should be equal to 0 and bonded to 0
    assert_eq!(query_user_info.shares.u128(), 0u128);
    assert_eq!(query_user_info.bonded.u128(), 0u128);

    //token1 balance should be equal to balence before deposit
    let user_balance_afeter = query_balance_native(&app, &Addr::unchecked(USER1), "token1");
    assert_eq!(user_balance_afeter.amount, Uint128::from(TOKEN_AMOUNT));

    //token2 balance should be equal to balence before deposit
    let user_balance_afeter = query_balance_native(&app, &Addr::unchecked(USER1), "token2");
    assert_eq!(user_balance_afeter.amount, Uint128::from(TOKEN_AMOUNT));



}

#[test]
fn unbond_in_parts_happy_path_test() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );

    //deposit assets
    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );
    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_ok());

    //Bond Tokens
    let msg = ExecuteMsg::Bond {};
    let bond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(bond_response.is_ok());

    //mock bond callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_1.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_2.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    //mock start unbond callback for both strategies
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount: TOKEN_HALF_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_HALF_AMOUNT);

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount:  TOKEN_HALF_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_HALF_AMOUNT);

    //update time
    update_time(&mut app, 101);

    //execute unbond
    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_1.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_ok());
    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_2.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_ok());

    //mock callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::UnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    mint_native(
        &mut app,
        contract_addr.to_string(),
        "token1".to_string(),
        1000000,
    );
    let start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond_callback_respose.is_ok());
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::UnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    mint_native(
        &mut app,
        contract_addr.to_string(),
        "token2".to_string(),
        1000000,
    );
    let start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond_callback_respose.is_ok());

    //user strategies should not decrees
    let query_user_info: UserResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::UserInfo {
                user: USER1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_user_info.strategies.len(), 2);

    //token1 balance should be equal to balence before deposit
    let user_balance_afeter = query_balance_native(&app, &Addr::unchecked(USER1), "token1");
    assert_eq!(user_balance_afeter.amount, Uint128::from(TOKEN_HALF_AMOUNT));

    //token2 balance should be equal to balence before deposit
    let user_balance_afeter = query_balance_native(&app, &Addr::unchecked(USER1), "token2");
    assert_eq!(user_balance_afeter.amount, Uint128::from(TOKEN_HALF_AMOUNT));

    //unbond rest of the tokens
    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    //mock start unbond callback for both strategies
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount: TOKEN_HALF_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_HALF_AMOUNT);
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount:  TOKEN_HALF_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_HALF_AMOUNT);

    //update time
    update_time(&mut app, 102);

    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_1.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_ok());
    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_2.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_ok());

    //mock callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::UnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    let start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond_callback_respose.is_ok());
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::UnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    let start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond_callback_respose.is_ok());

    //user strategies should decrees
    let query_user_info: UserResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::UserInfo {
                user: USER1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_user_info.strategies.len(), 0);

    //user shares should be equal to 0 and bonded to 0
    assert_eq!(query_user_info.shares.u128(), 0u128);
    assert_eq!(query_user_info.bonded.u128(), 0u128);

    //token1 balance should be equal to balence before deposit
    let user_balance_afeter = query_balance_native(&app, &Addr::unchecked(USER1), "token1");
    assert_eq!(user_balance_afeter.amount, Uint128::from(TOKEN_AMOUNT));

    //token2 balance should be equal to balence before deposit
    let user_balance_afeter = query_balance_native(&app, &Addr::unchecked(USER1), "token2");
    assert_eq!(user_balance_afeter.amount, Uint128::from(TOKEN_AMOUNT));
}

#[test]
fn start_unbond_in_parts_befor_async_complete_first_should_fail_test() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );

    //deposit assets
    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );
    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_ok());

    //Bond Tokens
    let msg = ExecuteMsg::Bond {};
    let bond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(bond_response.is_ok());

    //mock bond callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_1.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_2.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    //mock start unbond callback for both strategies
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount: TOKEN_HALF_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_HALF_AMOUNT);
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount: TOKEN_HALF_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_HALF_AMOUNT);

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_err());
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_err());
}

#[test]
fn multiple_start_unbond_before_receiving_callback_should_fail() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );

    //deposit assets
    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );
    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_ok());

    //Bond Tokens
    let msg = ExecuteMsg::Bond {};
    let bond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(bond_response.is_ok());

    //mock bond callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_1.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_2.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_err());

    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_err());
}

#[test]
fn unbond_before_receiving_start_unbond_callback_should_fail() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );

    //deposit assets
    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );
    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_ok());

    //Bond Tokens
    let msg = ExecuteMsg::Bond {};
    let bond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(bond_response.is_ok());

    //mock bond callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_1.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_2.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());
    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_1.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_err());
    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_2.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_err());
}

#[test]
fn deposit_bad_ratio_test() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );
    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );
    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        2000000u128.into(),
    );
    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_err());
}

#[test]
fn deposit_bad_token_test() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token3".to_string(),
        TOKEN_AMOUNT,
    );

    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token3".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );

    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_err());
}

#[test]
fn unbond_before_lock_time_pass_should_fail_test() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );

    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );

    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_ok());

    //Bond Tokens
    let msg = ExecuteMsg::Bond {};
    let bond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(bond_response.is_ok());

    //mock bond callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_1.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_2.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_1.to_string(),
        amount: TOKEN_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    let msg = ExecuteMsg::StartUnbond {
        id: BOND_ID_2.to_string(),
        amount: TOKEN_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_ok());

    //mock start unbond callback for both strategies
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse { share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_1.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_AMOUNT);

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::StartUnbondResponse {share_amount: TOKEN_AMOUNT.into(),
            unbond_id: BOND_ID_2.to_string(),
        },
    };
    let _start_unbond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    let query_bond_info: BondInfoResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &QueryMsg::BondInfo {
                id: BOND_ID_1.to_string(),
            },
        )
        .unwrap();
    assert_eq!(query_bond_info.unbonded.u128(), TOKEN_AMOUNT);

    let msg = ExecuteMsg::Unbond {
        id: BOND_ID_1.to_string(),
    };
    let unbond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(unbond_response.is_err());
}

#[test]
fn unbond_wrong_bond_id_should_fail_test() {
    let mut app = App::default();
    let (strategy1_addr, strategy2_addr) = create_strategies(&mut app, OWNER.to_owned());
    let contract_addr = create_bond_contract(
        &mut app,
        OWNER.to_owned(),
        "token1".to_owned(),
        "token2".to_owned(),
        strategy1_addr.clone(),
        strategy2_addr.clone(),
    );

    //mint some coins for user
    mint_native(
        &mut app,
        USER1.to_string(),
        "token1".to_string(),
        TOKEN_AMOUNT,
    );
    mint_native(
        &mut app,
        USER1.to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT,
    );

    let (msg, coins) = create_correct_deposit_msg(
        "token1".to_string(),
        "token2".to_string(),
        TOKEN_AMOUNT.into(),
        TOKEN_AMOUNT.into(),
    );

    let deposit_token =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &coins);
    assert!(deposit_token.is_ok());

    //Bond Tokens
    let msg = ExecuteMsg::Bond {};
    let bond_response =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(bond_response.is_ok());

    //mock bond callback
    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_1.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy1_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    let msg = ExecuteMsg::Callback {
        action: CallbackMsg::BondResponse {
            share_amount: TOKEN_AMOUNT.into(),
            bond_id: BOND_ID_2.to_string(),
        },
    };
    let bond_callback_respose =
        app.execute_contract(strategy2_addr.clone(), contract_addr.clone(), &msg, &[]);
    assert!(bond_callback_respose.is_ok());

    //execute start unbond
    let msg = ExecuteMsg::StartUnbond {
        id: "3".to_string(),
        amount: TOKEN_HALF_AMOUNT.into(),
    };
    let start_unbond =
        app.execute_contract(Addr::unchecked(USER1), contract_addr.clone(), &msg, &[]);
    assert!(start_unbond.is_err());
}


//Test helpers
fn create_strategies(app: &mut App, owner: String) -> (Addr, Addr) {
    let msg = MockInstantiateMsg {};
    let strategy = contract_strategy_mock1();
    let stratedy_code_id = app.store_code(strategy);
    let strategy2 = contract_strategy_mock2();
    let stratedy_code_id2 = app.store_code(strategy2);
    let strategy1_addr = app
        .instantiate_contract(
            stratedy_code_id,
            Addr::unchecked(owner.clone()),
            &msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let strategy2_addr = app
        .instantiate_contract(
            stratedy_code_id2,
            Addr::unchecked(owner),
            &msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();
    (strategy1_addr, strategy2_addr)
}

fn create_bond_contract(
    app: &mut App,
    owner: String,
    asset1: String,
    asset2: String,
    strategy1: Addr,
    strategy2: Addr,
) -> Addr {
    let instantiate_msg = InstantiateMsg {
        owner: owner.clone(),
        asset_infos: vec![
            AssetInfo::NativeToken {
                denom: asset1.to_string(),
            },
            AssetInfo::NativeToken {
                denom: asset2.to_string(),
            },
        ],
        strategy_infos: vec![strategy1, strategy2],
    };
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(owner),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();
    contract_addr
}

fn mint_native(app: &mut App, beneficiary: String, denom: String, amount: u128) {
    app.sudo(cw_multi_test::SudoMsg::Bank(
        cw_multi_test::BankSudo::Mint {
            to_address: beneficiary,
            amount: vec![coin(amount, denom)],
        },
    ))
    .unwrap();
}

fn query_balance_native(app: &App, address: &Addr, denom: &str) -> Coin {
    let req: QueryRequest<BankQuery> = QueryRequest::Bank(BankQuery::Balance {
        address: address.to_string(),
        denom: denom.to_string(),
    });
    let res = app.raw_query(&to_binary(&req).unwrap()).unwrap().unwrap();
    let balance: BalanceResponse = from_binary(&res).unwrap();
    return balance.amount;
}

fn update_time(app: &mut App, seconds: u64) {
    app.update_block(|block| {
        block.time = block.time.plus_seconds(seconds);
        block.height += 1
    });
}

fn create_correct_deposit_msg(
    asset1: String,
    asset2: String,
    amount_asset1: u128,
    amount_asset2: u128,
) -> (ExecuteMsg, [Coin; 2]) {
    let msg = ExecuteMsg::Deposit {
        assets: vec![
            Asset {
                info: AssetInfo::NativeToken {
                    denom: asset1.clone(),
                },
                amount: amount_asset1.clone().into(),
            },
            Asset {
                info: AssetInfo::NativeToken {
                    denom: asset2.clone(),
                },
                amount: amount_asset2.clone().into(),
            },
        ],
    };

    let coins = [
        Coin {
            denom: asset1.clone(),
            amount: amount_asset1.clone().into(),
        },
        Coin {
            denom: asset2.clone(),
            amount: amount_asset2.clone().into(),
        },
    ];
    (msg, coins)
}
