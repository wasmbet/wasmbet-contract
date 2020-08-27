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

use cosmwasm_std::{coins, from_binary, HandleResponse, HandleResult, InitResponse, StdError,Uint128};
use cosmwasm_vm::testing::{handle, init, mock_env, mock_instance, query};

use wasmbet_contract_dice::msg::{RoomStateResponse,StateResponse, HandleMsg, InitMsg, QueryMsg};

// This line will test the output of cargo wasm
static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/wasmbet_contract_dice.wasm");
// You can uncomment this line instead to test productionified build from rust-optimizer
// static WASM: &[u8] = include_bytes!("../contract.wasm");

#[test]
fn proper_initialization() {
    let mut deps = mock_instance(WASM, &[]);
    let seed = String::from("Hello, world!");
    let msg = InitMsg {
         seed: seed, 
         min_credit: Uint128::from(1000000u128), 
         max_credit: Uint128::from(10000000u128), 
         house_fee: 1,
        };
    let env = mock_env("creator", &coins(1000, "earth"));

    // we can just call .unwrap() to assert this was a success
    let res: InitResponse = init(&mut deps, env, msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // it worked, let's query the state
    let res = query(&mut deps, QueryMsg::Getstate{}).unwrap();
    let value: StateResponse = from_binary(&res).unwrap();
    assert_eq!(value.min_credit, 1000000 );
}
