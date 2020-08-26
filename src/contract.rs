use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,CanonicalAddr,Uint128,to_vec,HumanAddr,Coin , ReadonlyStorage
};
use serde_json_wasm as serde_json;
use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{State, room,ROOM_KEY,CONFIG_KEY,config, config_read};
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
        pot_pool: 0,
        fee_pool: 0, 
        seed: msg.seed.as_bytes().to_vec(),
        min_credit: msg.min_credit,
        max_credit: msg.max_credit,
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
//fn winer_payout
//fn owner_pot_deposit
//fn owner_pot_refund
//fn owner_fee_refund



fn can_deposit(env: &Env, state: &State, bet_amount: u128) -> StdResult<i64> {
    let deposit: Uint128;

    if env.message.sent_funds.len() == 0 {
        return Err(StdError::generic_err("SHOW ME THE MONEY"));
    } else {
        if env.message.sent_funds[0].denom != "ukrw" {
            return Err(StdError::generic_err("WRONG MONEY"));
        }
        deposit = env.message.sent_funds[0].amount;

        if deposit.u128() + bet_amount < state.min_credit {
            return Err(StdError::generic_err("GTFO DIRTY SHORT STACKER"));
        }

        if deposit.u128() + bet_amount > state.max_credit {
            return Err(StdError::generic_err("GTFO DIRTY DEEP STACKER"));
        }
    }
    Ok(deposit.u128() as i64)
}

pub fn try_ruler<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    phrase: String,
    prediction_number: i32,
    position: i32,
    bet_amount: &Uint128,
) -> StdResult<HandleResponse> {


    //1. pool amount check
    let state_pool = config_read(&deps.storage).load()?;
    if state_pool.pot_pool < 1 {
        return Err(StdError::generic_err(
            "Lack of reserves",
        ));
    }


    //2. position check
    if position > 2 {
        return Err(StdError::generic_err(
            "position expected, over and under",
        ));
    }


    //3. prediction check
    if prediction_number > 98 {
        return Err(StdError::generic_err(
            "prediction number, 4~98",
        ));
    }
    if prediction_number < 4 {
        return Err(StdError::generic_err(
            "prediction number, 4~98",
        ));
    }


    //4. dposit
    let amount_raw = bet_amount.u128();
    let mut state: State = config_read(&deps.storage).load()?;
    let deposit = can_deposit(&env, &state, amount_raw)?;
    state.pot_pool += deposit;
    config(&mut deps.storage).update(|mut state| {
            state.pot_pool += deposit;
            Ok(state)
    })?;


    //5.game state setting
    let mut room_store = PrefixedStorage::new(ROOM_KEY, &mut deps.storage);
    let raw_address = deps.api.canonical_address(&env.message.sender)?;
    let rand_entropy: Vec<u8> = Vec::new();


    //6. rand setting
    rand_entropy.extend(phrase.as_bytes());
    rand_entropy.extend(raw_address.as_slice().to_vec());
    rand_entropy.extend(env.block.chain_id.as_bytes().to_vec());
    rand_entropy.extend(&env.block.height.to_be_bytes());
    rand_entropy.extend(&env.block.time.to_be_bytes());
    rand_entropy = Sha256::digest(&rand_entropy).as_slice().to_vec();
    rand_entropy.extend_from_slice(&env.block.time.to_be_bytes());


    //7. lucky_number apply
    let mut state = config(&mut deps.storage).load()?;
    let mut rng: Prng = Prng::new(&state.seed, &rand_entropy);
    let Lucky_Number = rng.select_one_of(100);


    //8. prediction_num/lucky_num is position check
    // 0: over , 1: under
    // 0: win , 1: lose
    let winner = 1;
    match position {
        0 => {
            if Lucky_Number > prediction_number{
                winner = 0;
            }else{
                winner = 1;
            };
        },
        1 => {
            if Lucky_Number < prediction_number{
                winner = 0;
            }else{
                winner = 1;
            }
        }
    }
    //9. room state save
    let raw_room = to_vec(&room {
        start_time: env.block.time,
        entropy: Vec::default(),
        Prediction_number: prediction_number,
        Lucky_Number: Lucky_Number,
        position: position,
        results: winner,
        bet_amount: bet_amount,
    })?;

    room_store.set(raw_address.as_slice(), &raw_room); 

    //10. Distribution of rewards by win
        //11-1. Compensation ratio by number
        //11-2. Commission
        //11-3. rewards fund

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
