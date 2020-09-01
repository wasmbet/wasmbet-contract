use schemars::JsonSchema;
use cosmwasm_std::{HumanAddr,Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub seed : String,
    pub min_amount: Uint128,
    pub max_amount: Uint128,
    pub house_fee: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    TryPotPoolDeposit{
    },
    TryChangeMaxamount{
        max_amount: Uint128,
    },
    TryChangeMinamount{
        min_amount: Uint128,
    },
    TryChaingeFee{
        fee: u64,
    },
    TryPotPoolWithdraw{
        amount: Uint128
    },
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
    Getstate {},
    Getmystate {address:HumanAddr},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub contract_owner: HumanAddr,
    pub pot_pool: u64,
    pub min_amount: u64,
    pub max_amount: u64,
    pub house_fee: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RoomStateResponse {
    pub start_time: u64,
    pub entropy: Vec<u8>,
    pub prediction_number: u64,
    pub lucky_number: u64,
    pub position: String,
    pub win_results: bool,
    pub bet_amount: u64,
}
