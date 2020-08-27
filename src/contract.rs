use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,CanonicalAddr,Uint128,to_vec,HumanAddr,Coin , ReadonlyStorage ,BankMsg
};
use serde_json_wasm as serde_json;
use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{State, Room,ROOM_KEY,CONFIG_KEY,config, config_read};
use crate::rand::Prng;

use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use sha2::{Digest, Sha256};


pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    let state = State {
        contract_owner: deps.api.canonical_address(&env.message.sender)?,
        pot_pool: Uint128::from(0u128),
        fee_pool: Uint128::from(0u128), 
        seed: msg.seed.as_bytes().to_vec(),
        min_credit: msg.min_credit,
        max_credit: msg.max_credit,
        house_fee: 0.015,
    };
    config(&mut deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Ruler {phrase, prediction_number, position,bet_amount} => try_ruler(
            deps, 
            env,
            phrase,
            prediction_number,
            position,
            &bet_amount,
        ),
    }
}
//State Future features: change support
//fn change_min_credit
//fn change_max_credit
//fn owner_pot_deposit
//fn owner_pot_refund
//fn owner_fee_refund
//query pool blance
//query my_win_query


pub fn can_winer_payout(
    env: &Env,
    payout_amount: Uint128,
)-> StdResult<HandleResponse> {
    let token_transfer = BankMsg::Send {
        from_address: env.contract.address.clone(),
        to_address: env.message.sender.clone().clone(),
        amount: vec![Coin {
            denom: "ukrw".to_string(),
            amount: payout_amount,
        }],
    }
    .into();

    let res = HandleResponse {
        messages: vec![token_transfer],
        log: vec![
            log("action", "winner payout"),
        ],
        data: None,
    };

    Ok(res)
}

pub fn payout_amount(
    prediction_number: u32,
    position: String,
    bet_amount: &Uint128,
    fee: f64
) -> (f64,f64){
    let multiplier;
    let payout;
    let payout_fee;
    //98.5/99-Prediction=multiplier
    // ukrw =1000000 = 1krw 
    //98.5/Prediction=multiplier

    match &position[..] {
        "over" => {
            multiplier = 98.5/99.0-prediction_number as f64;
            let convert_bet_amount = *bet_amount;
            let float_bet_amount = convert_bet_amount.u128() as f64;
            let pay = float_bet_amount * multiplier ;
            payout_fee = float_bet_amount * multiplier * fee;
            payout = pay -payout_fee;
        },
        _ => {
            multiplier = 98.5/prediction_number as f64;
            let convert_bet_amount = *bet_amount;
            let float_bet_amount = convert_bet_amount.u128() as f64;
            let pay = float_bet_amount * multiplier;
            payout_fee = float_bet_amount * multiplier * fee;
            payout = pay -payout_fee;
        },
    }
    return (payout ,payout_fee)
}

pub fn try_ruler<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    phrase: String,
    prediction_number: u32,
    position: String,
    bet_amount: &Uint128,
) -> StdResult<HandleResponse> {


    //1. position check 
    if &position[..] == ""{
        return Err(StdError::generic_err(
            "position empty",
        ));
    }else if &position[..] != "under" && &position[..] != "over"{
        return Err(StdError::generic_err(
            "position not under/over",
        ));
    }

    //2. prediction check
    if &position[..] != "over"{
        if prediction_number < 4 && prediction_number > 98 {
            return Err(StdError::generic_err(
                "prediction number, 4~98",
            ));
        }
    }
    if &position[..] != "under"{
        if prediction_number < 1 && prediction_number > 95 {
            return Err(StdError::generic_err(
                "prediction number, 1~95",
            ));
        }
    }

    let state_pool = config_read(&deps.storage).load()?;
    //3.prediction check is pool amount check
    let (payout, payout_fee) = payout_amount(
        prediction_number,
        position.clone(), 
        bet_amount,
        state_pool.house_fee
    );

    if &position[..] == "over"{
        if state_pool.pot_pool < Uint128::from(payout as u128){
            return Err(StdError::generic_err("Lack of reserves",));
        }
    } else if &position[..] == "under"{
        if state_pool.pot_pool > Uint128::from(payout as u128){
            return Err(StdError::generic_err("Lack of reserves",));
        }
    }

    //4. user demon/amount check - Users should also double check
    //Minimum bet / maximum bet limit
    let mut amount_raw: Uint128 = Uint128::default();
    for coin in &env.message.sent_funds {
        if coin.denom == "ukrw" {
            amount_raw = coin.amount
        } else{
            return Err(StdError::generic_err(format!(
                "Insufficient ukrw denom",
            )));
        }
    }
    if amount_raw != *bet_amount {
        return Err(StdError::generic_err(format!(
            "Insufficient ukrw deposit: bet_amount={}, required={}",
            *bet_amount, amount_raw
        )));
    } else if env.message.sent_funds.len() == 0{
        return Err(StdError::generic_err("SHOW ME THE MONEY"));
    }
    if *bet_amount < state_pool.min_credit {
        return Err(StdError::generic_err("GTFO DIRTY SHORT STACKER"));
    }

    if *bet_amount > state_pool.max_credit {
        return Err(StdError::generic_err("GTFO DIRTY DEEP STACKER"));
    }

    //5.game state setting
    let mut room_store = PrefixedStorage::new(ROOM_KEY, &mut deps.storage);
    let raw_address = deps.api.canonical_address(&env.message.sender)?;
    let mut rand_entropy: Vec<u8> = Vec::new();


    //6. rand setting
    rand_entropy.extend(phrase.as_bytes());
    rand_entropy.extend(raw_address.as_slice().to_vec());
    rand_entropy.extend(env.block.chain_id.as_bytes().to_vec());
    rand_entropy.extend(&env.block.height.to_be_bytes());
    rand_entropy.extend(&env.block.time.to_be_bytes());
    rand_entropy = Sha256::digest(&rand_entropy).as_slice().to_vec();
    rand_entropy.extend_from_slice(&env.block.time.to_be_bytes());


    //7. lucky_number apply
    let mut rng: Prng = Prng::new(&state_pool.seed, &rand_entropy);
    let lucky_number = rng.select_one_of(99);


    //8. prediction_num/lucky_num is position check
    // 0: win , 1: lose
    // 98.5/prediction_number
    let win_results;
    match &position[..] {
        "over" => {
            if lucky_number >= prediction_number{
                win_results = 0;
            }else{
                win_results = 1;
            };
        },
        "under" => {
            if lucky_number <= prediction_number{
                win_results = 0;
            }else{
                win_results = 1;
            }
        },
        _ => {
            return Err(StdError::generic_err(
                "position invalid",
            ));
        }
    }
    //9. room state save
    let raw_room = to_vec(&Room {
        start_time: env.block.time,
        entropy: rand_entropy,
        prediction_number: prediction_number,
        lucky_number: lucky_number,
        position: position,
        results: win_results,
        bet_amount: *bet_amount,
    })?;

    room_store.set(raw_address.as_slice(), &raw_room); 

    //10. Distribution of rewards by win and lose
        //11-1. Compensation ratio by number
        //11-2. fee
        //11-3. rewards fund
    let convert_bet_amount = *bet_amount;
    let float_bet_amount = convert_bet_amount.u128() as f64;
    let fee = float_bet_amount-float_bet_amount * state_pool.house_fee;  
    let convert_fee= Uint128::from(fee as u128);

    config(&mut deps.storage).update(|mut state| {
        state.fee_pool += convert_fee;
        Ok(state)
    })?;
    if win_results == 1{
        let bet_amount_fee = *bet_amount - convert_fee;
        config(&mut deps.storage).update(|mut state| {
            state.pot_pool += bet_amount_fee.unwrap();
            Ok(state)
        })?;
    }else if win_results == 0{
        config(&mut deps.storage).update(|mut state| {
            state.fee_pool += Uint128::from(payout_fee as u128);
            Ok(state)
        })?;

        if state_pool.pot_pool < Uint128::from(payout as u128){
            can_winer_payout(&env, *bet_amount);
            return Err(StdError::generic_err(
                "Lack of reserves, bet_amount refund",
            ));
        } else if state_pool.pot_pool > Uint128::from(payout as u128){
            let potout = state_pool.pot_pool.u128()- payout as u128;
            can_winer_payout(&env, Uint128::from(payout as u128));
            config(&mut deps.storage).update(|mut state| {
                state.pot_pool =Uint128::from(potout);
                Ok(state)
            })?;
        }
    }

    Ok(HandleResponse::default())
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // beneficiary can release it
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // beneficiary can release it
        let unauth_env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let res = handle(&mut deps, unauth_env, msg);
        match res {
            Err(StdError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_env = mock_env("creator", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let _res = handle(&mut deps, auth_env, msg).unwrap();

        // should now be 5
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }
}
