use schemars::JsonSchema;
use cosmwasm_std::{HumanAddr,Uint128};
use serde::{Deserialize, Serialize};
use crate::state::{Casino, Stakes};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum InitMsg {
    CreateCasino { 
        name: String,
        description: String,
        min_bet_amount: Uint128,
        max_bet_rate: u64,
        house_fee: u64,
        founder_commission_rate: u64,
        seed : String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    TryCapitalDeposit{},
    TryCapitalWithdraw{},
    Ruler {
        phrase: String,
        prediction_number: u64,
        position: String,
        bet_amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetCasinoStakeInfo {},
    GetCasinoInfo {},
    GetStakeInfo {},
    Getmystate {address:HumanAddr},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoomStateResponse {
    pub start_time: u64,
    pub entropy: Vec<u8>,
    pub prediction_number: u64,
    pub lucky_number: u64,
    pub position: String,
    pub results: bool,
    pub payout: u64,
    pub bet_amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CasinoResponse {
    pub casino: Casino,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakeResponse {
    pub stake: Stakes,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CasinoStakeResponse {
    pub casino: Casino,
    pub stake: Stakes,
}