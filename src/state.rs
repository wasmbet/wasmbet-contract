use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";
//pub static ROOM_KEY: &[u8] = b"room";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Room_State {
    pub room_owner: CanonicalAddr,
    pub seed : Vec<u8>,
    pub start_time: u64,
    pub entropy: Vec<u8>,
    pub Prediction_number: i32,
    pub Lucky_Number: i32,
    pub position: String,
    pub results: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub contract_owner: CanonicalAddr,
    pub player_room: Vec<Room_State>,
}


pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

//pub fn room<S: Storage>(storage: &mut S) -> Singleton<S, State> {
//    singleton(storage, ROOM_KEY)
//}

//pub fn room_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
//    singleton_read(storage, ROOM_KEY)
//}
