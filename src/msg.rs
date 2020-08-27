use schemars::JsonSchema;
use cosmwasm_std::{HumanAddr,Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub seed : String,
    pub min_credit: Uint128,
    pub max_credit: Uint128,
    pub house_fee: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    TryPotPoolDeposit{
    },
    TryChangeMaxcredit{
        max_credit: Uint128,
    },
    TryChangeMincredit{
        min_credit: Uint128,
    },
    TryChaingeFee{
        fee: f64,
    },
    TryFeePoolWithdraw{
        amount: Uint128
    },
    TryPotPoolWithdraw{
        amount: Uint128
    },

    Ruler {
        phrase: String,
        prediction_number: u32,
        position: String,
        bet_amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    Getstate {},
    GetMyRoomState {address:HumanAddr},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub contract_owner: HumanAddr,
    pub pot_pool: u128,
    pub fee_pool: u128,
    pub min_credit: u128,
    pub max_credit: u128,
    pub house_fee: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoomStateResponse {
    pub start_time: u64,
    pub entropy: Vec<u8>,
    pub prediction_number: u32,
    pub lucky_number: u32,
    pub position: String,
    pub results: u8,
    pub bet_amount: u128,
}
