use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Uint128};

pub const CONFIG_KEY: &[u8] = b"config";
pub const ROOM_KEY: &[u8] = b"room";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Room {
    pub start_time: u64,
    pub entropy: Vec<u8>,
    pub prediction_number: u64,
    pub lucky_number: u64,
    pub position: String,
    pub results: u64,
    pub bet_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub contract_owner: CanonicalAddr,
    pub pot_pool: Uint128,
    pub fee_pool: Uint128,
    pub seed : Vec<u8>,
    pub min_credit: Uint128,
    pub max_credit: Uint128,
    pub house_fee: u64,
}