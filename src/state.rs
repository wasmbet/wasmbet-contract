use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128};

pub const ROOM_KEY: &[u8] = b"room";

pub const CASINO_KEY: &[u8] = b"casino";
pub const STAKE_KEY: &[u8] = b"stake";
pub const RESULT_KEY: &[u8] = b"result";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Casino {
    pub founder: HumanAddr,
    pub name: String,
    pub description: String,
    pub min_bet_amount: Uint128,
    pub max_bet_rate: u64,
    pub house_fee: u64,
    pub founder_commission_rate: u64,
    pub capital: Uint128,
    pub bet_cumulative_amount: Uint128,
    pub seed : Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Result {
    pub owner: HumanAddr,
    pub prediction_number: u64,
    pub lucky_number: u64,
    pub win_results: bool,
    pub position: String,
    pub bet_amount: Uint128,
    pub payout: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakeInfo {
    pub address: HumanAddr,
    pub begin_amount: Uint128,
    pub ownership_percentage : Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Room {
    pub start_time: u64,
    pub entropy: Vec<u8>,
    pub prediction_number: u64,
    pub lucky_number: u64,
    pub position: String,
    pub results: bool,
    pub payout: Uint128,
    pub bet_amount: Uint128,
}

pub type Results = Vec<Result>;
pub type Stakes = Vec<StakeInfo>;