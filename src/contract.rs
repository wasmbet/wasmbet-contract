use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, Uint128, to_vec, Coin, ReadonlyStorage, from_slice, HumanAddr, BankMsg,
};
use crate::msg::{RoomStateResponse, StateResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{State, Room, ROOM_KEY, CONFIG_KEY};
use crate::rand::Prng;
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use sha2::{Digest, Sha256};
use serde_json_wasm as serde_json;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        contract_owner: deps.api.canonical_address(&env.message.sender)?,
        pot_pool: Uint128::from(0u128),
        seed: msg.seed.as_bytes().to_vec(),
        min_amount: msg.min_amount,
        max_amount: msg.max_amount,
        house_fee: msg.house_fee,
    };
    deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
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
        HandleMsg::TryPotPoolDeposit{} => try_pot_pool_deposit(
            deps, 
            env,
        ),
        HandleMsg::TryChangeMaxamount{max_amount} => try_change_maxamount(
            deps, 
            env, 
            &max_amount,
        ),
        HandleMsg::TryChangeMinamount{min_amount} => try_change_minamount(
            deps, 
            env, 
            &min_amount,
        ),
        HandleMsg::TryChaingeFee{fee} => try_change_fee(
            deps,
            env,
            fee,
        ),
        HandleMsg::TryPotPoolWithdraw{amount} => try_pot_pool_withdraw(
            deps,
            env,
            &amount,
        ),
    }
}
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Getstate {} => to_binary(
            &read_state(
                deps
            )?
        ),
        QueryMsg::Getmystate{address}=> to_binary(
            &read_root_state(
                &address,
                &deps.storage,
                &deps.api
            )?
        )
    }
}
fn try_pot_pool_deposit<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut amount_raw: Uint128 = Uint128::default();

    for coin in &env.message.sent_funds {
        if coin.denom == "ukrw" {
            amount_raw = coin.amount
        }
    }

    if amount_raw == Uint128::default() {
        return Err(StdError::generic_err(format!("Lol send some funds dude")));
    }

    let api = &deps.api;
    let mut state: State = serde_json::from_slice(&deps.storage.get(CONFIG_KEY).unwrap()).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
            return Err(StdError::generic_err(format!("not owner address")));
    }
    state.pot_pool += amount_raw;
    deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
    Ok(HandleResponse::default())
}
fn try_change_maxamount<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    max_amount: &Uint128,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut state: State = serde_json::from_slice(&deps.storage.get(CONFIG_KEY).unwrap()).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    state.max_amount = *max_amount;
    deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
    Ok(HandleResponse::default())
}
fn try_change_minamount<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    min_amount: &Uint128,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut state: State = serde_json::from_slice(&deps.storage.get(CONFIG_KEY).unwrap()).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    state.min_amount = *min_amount;
    deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
    Ok(HandleResponse::default())
}
fn try_change_fee<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    fee: u64,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut state: State = serde_json::from_slice(&deps.storage.get(CONFIG_KEY).unwrap()).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    state.house_fee = fee;
    deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
    Ok(HandleResponse::default())
}

fn try_pot_pool_withdraw<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut state: State = serde_json::from_slice(&deps.storage.get(CONFIG_KEY).unwrap()).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    if state.pot_pool < *amount{
        return Err(StdError::generic_err(format!("insufficient pot pool")));
    } else if state.pot_pool > *amount{
        let payaout = state.pot_pool - *amount;
        state.pot_pool = payaout.unwrap();
    }
    let transfer = can_winer_payout(&env, *amount).unwrap();
    deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
    Ok(transfer)
}

pub fn can_winer_payout(
    env : &Env,
    amount: Uint128,
)-> StdResult<HandleResponse> {
    let token_transfer = BankMsg::Send {
        from_address: env.contract.address.clone(),
        to_address: env.message.sender.clone(),
        amount: vec![Coin {
            denom: "ukrw".to_string(),
            amount: amount,
        }],
    }
    .into();
    let res = HandleResponse {
        messages: vec![token_transfer],
        log: vec![
            log("action", "transfer payout"),
        ],
        data: None,
    };

    Ok(res)
}
pub fn payout_amount(
    prediction_number: u64,
    position: String,
    bet_amount: &Uint128,
    fee: u64
) -> StdResult<u128>{
    let multiplier : u128;
    let payout;
    //98.5/99-Prediction=multiplier
    // ukrw =1000000 = 1krw 
    //98.5/Prediction=multiplier

    match &position[..] {
        "over" => {
            multiplier = (1000000 as u128- fee as u128)/(99 as u128-(prediction_number as u128*5/3));
            let bet_amount = *bet_amount;
            payout = bet_amount.u128() * multiplier/10000;
        },
        _ => {
            //(1000000 - 1500) / 30 x 5 / 3 = 998500 / 50 = 19970
            multiplier = (1000000 as u128- fee as u128)/(prediction_number as u128*5/3);
            let bet_amount = *bet_amount;
            // 1000000 x 19970/10000
            payout = bet_amount.u128() * multiplier/10000;
        },
    }
    Ok(payout)
}
pub fn try_ruler<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    phrase: String,
    prediction_number: u64,
    position: String,
    bet_amount: &Uint128,
) -> StdResult<HandleResponse> {


    //1. position check 
    if &position[..] != "under" && &position[..] != "over"{
        return Err(StdError::generic_err(
            "position not under/over",
        ));
    }

    //2. prediction check
    if &position[..] == "over"{
        if prediction_number < 2 || prediction_number > 58 {
            return Err(StdError::generic_err(
                "prediction number, 2~58",
            ));
        }
    } else if &position[..] == "under"{
        if prediction_number < 1 || prediction_number > 57 {
            return Err(StdError::generic_err(
                "prediction number, 1~57",
            ));
        }
    }
    let mut state: State = serde_json::from_slice(&deps.storage.get(CONFIG_KEY).unwrap()).unwrap();
    
    //3.prediction check is pool amount check
    
    let payout = payout_amount(
        prediction_number,
        position.clone(), 
        bet_amount,
        state.house_fee
    )?;

    if state.pot_pool < Uint128::from(payout){
        return Err(StdError::generic_err(format!("Lack of reserves pot={}, payout={}, bet={}",state.pot_pool, payout,*bet_amount)));
    }
    
    //4. user demon/amount check - Users should also double check
    //Minimum bet / maximum bet limit
    if env.message.sent_funds.len() == 0{
        return Err(StdError::generic_err("There is no money in the wallet"));
    }

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
            "Insufficient ukrw set amount: bet_amount={}, required={}",
            *bet_amount, amount_raw
        )));
    }

    if *bet_amount < state.min_amount {
        return Err(StdError::generic_err("Below the minimum bet amount."));
    }

    if *bet_amount > state.max_amount {
        return Err(StdError::generic_err("The maximum bet amount is exceeded."));
    }
    
    //5.game state setting
    //let mut room_store = PrefixedStorage::new(ROOM_KEY, &mut deps.storage);
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
    let mut rng: Prng = Prng::new(&state.seed, &rand_entropy);

    let lucky_number_u32 = rng.select_one_of(59);
    let lucky_number = lucky_number_u32 as u64;

    //8. prediction_num/lucky_num is position check
    // true: win , false: lose
    // 98.5/prediction_number
    let win_results;
    match &position[..] {
        "over" => {
            if lucky_number > prediction_number{
                win_results = true;
            }else{
                win_results = false;
            };
        },
        "under" => {
            if lucky_number < prediction_number{
                win_results = true;
            }else{
                win_results = false;
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
        win_results: win_results,
        bet_amount: *bet_amount,
    })?;
    let mut room_store = PrefixedStorage::new(ROOM_KEY, &mut deps.storage);
    room_store.set(raw_address.as_slice(), &raw_room); 

    //10. Distribution of rewards by win and lose
    if win_results == false{
        state.pot_pool += *bet_amount;
        deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
    }else if win_results == true{
        if state.pot_pool < Uint128::from(payout as u128){
            deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
            return Err(StdError::generic_err(
                "Lack of reserves, bet_amount refund",
            ));
        } else if state.pot_pool > Uint128::from(payout as u128){
            let potout = state.pot_pool.u128()- payout as u128;
            let send_result : HandleResponse = can_winer_payout(&env, Uint128::from(payout as u128)).unwrap();
            state.pot_pool = Uint128::from(potout);
            deps.storage.set(CONFIG_KEY, &serde_json::to_vec(&state).unwrap());
            return Ok(send_result)
            
        }
    }
    Ok(HandleResponse::default())
}
fn read_state<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> StdResult<StateResponse> {
    let state: State = serde_json::from_slice(&deps.storage.get(CONFIG_KEY).unwrap()).unwrap();
    let owner = deps.api.human_address(&state.contract_owner)?;
    let pot = state.pot_pool.u128();
    let min_amount = state.min_amount.u128();
    let max_amount = state.max_amount.u128();
    Ok(StateResponse{
        contract_owner: owner,
        pot_pool: pot as u64,
        min_amount: min_amount as u64,
        max_amount: max_amount as u64,
        house_fee: state.house_fee,
    })
}
fn read_root_state<S: Storage, A: Api>(
    address: &HumanAddr,
    store: &S,
    api: &A,
) -> StdResult<RoomStateResponse> {
    let owner_address = api.canonical_address(address)?;
    let room_store = ReadonlyPrefixedStorage::new(ROOM_KEY, store);
    let room_state = room_store.get(owner_address.as_slice()).unwrap();
    let room : Room = from_slice(&room_state).unwrap();
    let amount = room.bet_amount.u128();
    Ok(RoomStateResponse{
        start_time: room.start_time,
        entropy: room.entropy,
        prediction_number: room.prediction_number,
        lucky_number: room.lucky_number,
        position: room.position,
        win_results: room.win_results,
        bet_amount: amount as u64,
    })
}
