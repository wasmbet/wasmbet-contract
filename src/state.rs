use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
pub static ROOM_KEY: &[u8] = b"room";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct room {
    pub start_time: u64,
    pub entropy: Vec<u8>,
    pub Prediction_number: i32,
    pub Lucky_Number: i32,
    pub position: i32,
    pub results: i32,
    pub bet_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub contract_owner: CanonicalAddr,
    pub pot_pool: i64,
    pub fee_pool: i64,
    pub seed : Vec<u8>,
    pub min_credit: u128,
    pub max_credit: u128,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
