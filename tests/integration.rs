//! This integration test tries to run and call the generated wasm.
//! It depends on a Wasm build being available, which you can create with `cargo wasm`.
//! Then running `cargo integration-test` will validate we can properly call into that generated Wasm.
//!
//! You can easily convert unit tests to integration tests.
//! 1. First copy them over verbatum,
//! 2. Then change
//!      let mut deps = mock_dependencies(20, &[]);
//!    to
//!      let mut deps = mock_instance(WASM, &[]);
//! 3. If you access raw storage, where ever you see something like:
//!      deps.storage.get(CONFIG_KEY).expect("no data stored");
//!    replace it with:
//!      deps.with_storage(|store| {
//!          let data = store.get(CONFIG_KEY).expect("no data stored");
//!          //...
//!      });
//! 4. Anywhere you see query(&deps, ...) you must replace it with query(&mut deps, ...)

use cosmwasm_std::{ HumanAddr,coins, from_binary, HandleResponse, HandleResult, InitResponse, StdError,Uint128};
use cosmwasm_vm::testing::{handle, init, mock_env, mock_instance, query};

use wasmbet_contract_timeroulette::msg::{RoomStateResponse,StateResponse, HandleMsg, InitMsg, QueryMsg};

// This line will test the output of cargo wasm
static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/wasmbet_contract_timeroulette.wasm");
// You can uncomment this line instead to test productionified build from rust-optimizer
// static WASM: &[u8] = include_bytes!("../contract.wasm");

// to be revised
#[test]
fn proper_initialization() {
    let mut deps = mock_instance(WASM, &[]);
    let seed = String::from("Hello, world!");
    let fee = 15000 as u64;
    let msg = InitMsg {
         seed: seed, 
         min_credit: Uint128::from(1000000u128), 
         max_credit: Uint128::from(10000000u128), 
         house_fee: fee,
        };
    let env = mock_env("creator", &coins(10000000000000000, "ukrw"));

    // we can just call .unwrap() to assert this was a success
    let res: InitResponse = init(&mut deps, env, msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // it worked, let's query the state
    let res = query(&mut deps, QueryMsg::Getstate{}).unwrap();
    let value: StateResponse = from_binary(&res).unwrap();
    //assert_eq!(value.contract_owner, "creator" );
    //assert_eq!(value.pot_pool, 100000000000000 );
    //assert_eq!(value.seed, "Hello, world!" );
    assert_eq!(value.min_credit, 1000000 );
    assert_eq!(value.max_credit, 10000000 );
    //assert_eq!(value.house_fee, 1 );
    
    let mut env = mock_env("creator", &coins(2000000, "ukrw"));
    env.block.height = 2;
    env.block.time = 1232342344153;
    let env2 = mock_env("creator2", &coins(2000000, "ukrw"));
    let seed = String::from("Hello, world!123312");
    let seed2 = String::from("Hello, world!2");
    let under = String::from("under");
    let over = String::from("under");
    let ruler = HandleMsg::Ruler {
            phrase: seed,
            prediction_number:50,
            position: under,
            bet_amount: Uint128::from(2000000u128),
    };
    let ruler2 = HandleMsg::Ruler {
            phrase: seed2,
            prediction_number:50,
            position: over,
            bet_amount: Uint128::from(2000000u128),
    };
    let try_ruler_response: HandleResponse = handle(&mut deps, env, ruler).unwrap();
    assert_eq!(try_ruler_response.messages.len(), 0);
    let try_ruler_response2: HandleResponse = handle(&mut deps, env2, ruler2).unwrap();
    assert_eq!(try_ruler_response2.messages.len(), 0);
    // it worked, let's query the state
    
    let addres = HumanAddr("creator".to_string());
    let addres2 = HumanAddr("creator2".to_string());
    let res = query(&mut deps, QueryMsg::GetMyRoomState{address:addres}).unwrap();
    let value: RoomStateResponse = from_binary(&res).unwrap();
    let res2 = query(&mut deps, QueryMsg::GetMyRoomState{address:addres2}).unwrap();
    let value2: RoomStateResponse = from_binary(&res2).unwrap();
    let res3 = query(&mut deps, QueryMsg::Getstate{}).unwrap();
    let value3: StateResponse = from_binary(&res3).unwrap();
    //assert_eq!(value.contract_owner, "creator" );
    assert_eq!(value.lucky_number, 99999998079400 );
    //assert_eq!(value2.results, 99999998079400 );
    //assert_eq!(value3.pot_pool, 99999998079400 );
    //assert_eq!(value.seed, "Hello, world!" );
    //assert_eq!(value3.min_credit, 1000000 );
    //assert_eq!(value3.max_credit, 10000000 );
    //assert_eq!(value3.house_fee, 1 );
    //assert_eq!(value.prediction_number, 50 );
    //assert_eq!(value.position, "under" );
    //assert_eq!(value.bet_amount, 2000000 );
    //assert_eq!(value2.prediction_number, 50 );
    //assert_eq!(value2.position, "over" );
    //assert_eq!(value2.bet_amount, 2000000 );


}