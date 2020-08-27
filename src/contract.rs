use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, Uint128, to_vec, Coin, CosmosMsg, ReadonlyStorage, from_slice, HumanAddr, BankMsg,
};
use crate::msg::{RoomStateResponse, StateResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{State, Room, ROOM_KEY, config_read, CONFIG_KEY, KEY_CONSTANTS};
use crate::rand::Prng;
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use sha2::{Digest, Sha256};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let state = to_vec(&State {
        contract_owner: deps.api.canonical_address(&env.message.sender)?,
        pot_pool: Uint128::from(0u128),
        fee_pool: Uint128::from(0u128), 
        seed: msg.seed.as_bytes().to_vec(),
        min_credit: msg.min_credit,
        max_credit: msg.max_credit,
        house_fee: 1,
    })?;
    config_store.set(KEY_CONSTANTS, &state);
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
        HandleMsg::TryChangeMaxcredit{max_credit} => try_change_maxcredit(
            deps, 
            env, 
            &max_credit,
        ),
        HandleMsg::TryChangeMincredit{min_credit} => try_change_mincredit(
            deps, 
            env, 
            &min_credit,
        ),
        HandleMsg::TryChaingeFee{fee} => try_change_fee(
            deps,
            env,
            fee,
        ),
        HandleMsg::TryFeePoolWithdraw{amount} => try_fee_pool_withdraw(
            deps,
            env,
            &amount,
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
        QueryMsg::GetMyRoomState{address}=> to_binary(
            &read_root_state(
                address,
                &deps.storage,
                &deps.api
            )?
        )
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PayResponse{
    pub payout : u64,
    pub payout_fee: u64
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
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let data = config_store.get(KEY_CONSTANTS).expect("no pot_pool_deposit");
    let mut state : State = from_slice(&data).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
            return Err(StdError::generic_err(format!("not owner address")));
    }
    state.pot_pool += amount_raw;
    config_store.set(KEY_CONSTANTS, &to_vec(&state).unwrap());    

    Ok(HandleResponse::default())
}
fn try_change_maxcredit<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    max_credit: &Uint128,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let data = config_store.get(KEY_CONSTANTS).expect("no change_maxcredit");
    let mut state : State = from_slice(&data).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    state.min_credit = *max_credit;
    config_store.set(KEY_CONSTANTS, &to_vec(&state).unwrap()); 
    Ok(HandleResponse::default())
}
fn try_change_mincredit<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    min_credit: &Uint128,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let data = config_store.get(KEY_CONSTANTS).expect("no change_mincredit");
    let mut state : State = from_slice(&data).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    state.min_credit = *min_credit;
    config_store.set(KEY_CONSTANTS, &to_vec(&state).unwrap()); 
    Ok(HandleResponse::default())
}
fn try_change_fee<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    fee: u64,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let data = config_store.get(KEY_CONSTANTS).expect("no pot_pool_deposit");
    let mut state : State = from_slice(&data).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    state.house_fee = fee;
    config_store.set(KEY_CONSTANTS, &to_vec(&state).unwrap()); 
    Ok(HandleResponse::default())
}
fn try_fee_pool_withdraw<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let data = config_store.get(KEY_CONSTANTS).expect("no pot_pool_deposit");
    let mut state : State = from_slice(&data).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    if state.fee_pool < *amount{
        return Err(StdError::generic_err(format!("insufficient fee pool")));
    } else if state.fee_pool > *amount{
            let payaout = state.fee_pool - *amount;
            state.fee_pool = payaout.unwrap();
    }
    config_store.set(KEY_CONSTANTS, &to_vec(&state).unwrap());  
    let transfer = can_winer_payout(&env, *amount).unwrap();
    let res = HandleResponse {
        messages: vec![transfer],
        log: vec![
            log("action", "pot_pool_withdraw"),
        ],
        data: None,
    };
    Ok(res)
}
fn try_pot_pool_withdraw<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    amount: &Uint128,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let data = config_store.get(KEY_CONSTANTS).expect("no pot_pool_withdraw");
    let mut state : State = from_slice(&data).unwrap();
    if api.canonical_address(&env.message.sender)? != state.contract_owner {
        return Err(StdError::generic_err(format!("not owner address")));
    }
    if state.pot_pool < *amount{
        return Err(StdError::generic_err(format!("insufficient fee pool")));
    } else if state.pot_pool > *amount{
        let payaout = state.pot_pool - *amount;
        state.pot_pool = payaout.unwrap();
        config_store.set(KEY_CONSTANTS, &to_vec(&state).unwrap());
    }

    let transfer = can_winer_payout(&env, *amount).unwrap();
    let res = HandleResponse {
        messages: vec![transfer],
        log: vec![
            log("action", "pot_pool_withdraw"),
        ],
        data: None,
    };
    Ok(res)
}
pub fn can_winer_payout(
    env: &Env,
    payout_amount: Uint128,
)-> StdResult<CosmosMsg> {
    let token_transfer = BankMsg::Send {
        from_address: env.contract.address.clone(),
        to_address: env.message.sender.clone(),
        amount: vec![Coin {
            denom: "ukrw".to_string(),
            amount: payout_amount,
        }],
    }
    .into();
    Ok(token_transfer)
}
pub fn payout_amount(
    prediction_number: u64,
    position: String,
    bet_amount: &Uint128,
    fee: u64
) -> StdResult<PayResponse>{
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
            let fee_convert = fee as f64 / 100.0;
            payout_fee = float_bet_amount * multiplier * fee_convert;
            payout = pay -payout_fee;
        },
        _ => {
            multiplier = 98.5/prediction_number as f64;
            let convert_bet_amount = *bet_amount;
            let float_bet_amount = convert_bet_amount.u128() as f64;
            let pay = float_bet_amount * multiplier;
            let fee_convert = fee as f64 / 100.0;
            payout_fee = float_bet_amount * multiplier * fee_convert;
            payout = pay -payout_fee;
        },
    }
    Ok(PayResponse{payout: payout as u64, payout_fee: payout_fee as u64})
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
    let payout_struct = payout_amount(
        prediction_number,
        position.clone(), 
        bet_amount,
        state_pool.house_fee
    );
    let payout_unwrap = payout_struct.unwrap();
    let payout = payout_unwrap.payout;
    let payout_fee = payout_unwrap.payout_fee;
   
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
    let lucky_number_u32 = rng.select_one_of(99);
    let lucky_number = lucky_number_u32 as u64;


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
    let convert_fee = state_pool.house_fee as f64 / 100.0;
    let fee = float_bet_amount-float_bet_amount * convert_fee;  
    let convert_fee= Uint128::from(fee as u128);
    let mut config_store = PrefixedStorage::new(CONFIG_KEY, &mut deps.storage);
    let data = config_store.get(KEY_CONSTANTS).expect("no pot_pool_deposit");
    let mut state : State = from_slice(&data).unwrap();
    state.fee_pool += convert_fee;

    if win_results == 1{
        let bet_amount_fee = *bet_amount - convert_fee;
        state.pot_pool += bet_amount_fee.unwrap();
    }else if win_results == 0{
            state.fee_pool += Uint128::from(payout_fee as u128);
        if state_pool.pot_pool < Uint128::from(payout as u128){
            let _ = can_winer_payout(&env, *bet_amount);
            return Err(StdError::generic_err(
                "Lack of reserves, bet_amount refund",
            ));

        } else if state_pool.pot_pool > Uint128::from(payout as u128){
            let potout = state_pool.pot_pool.u128()- payout as u128;
            let _ = can_winer_payout(&env, Uint128::from(payout as u128));
            state.pot_pool =Uint128::from(potout);
            
        }
    }
    config_store.set(KEY_CONSTANTS, &to_vec(&state).unwrap()); 
    Ok(HandleResponse::default())
}
fn read_state<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> StdResult<StateResponse> {
    let state = config_read(&deps.storage).load()?;
    let owner = deps.api.human_address(&state.contract_owner)?;
    let pot = state.pot_pool.u128();
    let fee_pool = state.fee_pool.u128();
    let min_credit = state.min_credit.u128();
    let max_credit = state.max_credit.u128();
    Ok(StateResponse{
        contract_owner: owner,
        pot_pool: pot,
        fee_pool: fee_pool,
        min_credit:min_credit,
        max_credit: max_credit,
        house_fee: state.house_fee,
    })
}
fn read_root_state<S: Storage, A: Api>(
    address: HumanAddr,
    store: &S,
    api: &A,
) -> StdResult<RoomStateResponse> {
    let owner_address = api.canonical_address(&address)?;
    let room_store = ReadonlyPrefixedStorage::new(ROOM_KEY, store);
    let room_state = room_store.get(owner_address.as_slice()).unwrap();
    let room : Room = from_slice(&room_state).unwrap();
    Ok(RoomStateResponse{
        start_time: room.start_time,
        entropy: room.entropy,
        prediction_number: room.prediction_number,
        lucky_number: room.lucky_number,
        position: room.position,
        results: room.results,
        bet_amount: room.bet_amount.u128(),
    })
}