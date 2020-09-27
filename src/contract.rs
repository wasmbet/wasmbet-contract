use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, Uint128, to_vec, Coin, CosmosMsg,ReadonlyStorage, from_slice, HumanAddr, BankMsg,
};
use crate::msg::{RoomStateResponse, CasinoResponse, StakeResponse, CasinoStakeResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{Casino, Results, Stakes, StakeInfo, Room, ROOM_KEY, CASINO_KEY, STAKE_KEY, RESULT_KEY};
use crate::rand::Prng;
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use sha2::{Digest, Sha256};
use serde_json_wasm as serde_json;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    match msg {
        InitMsg::CreateCasino { 
            name,
            description,
            min_bet_amount,
            max_bet_rate, 
            house_fee, 
            founder_commission_rate,
            seed
        } => {

            let casino = Casino {
                founder: env.message.sender,
                name,
                description,
                min_bet_amount,
                max_bet_rate,
                house_fee,
                founder_commission_rate,
                capital: Uint128::from(0u128),
                seed: seed.as_bytes().to_vec(),
                bet_cumulative_amount: Uint128::from(0u128),
            };

            deps.storage.set(CASINO_KEY, &serde_json::to_vec(&casino).unwrap());
            deps.storage.set(STAKE_KEY, &serde_json::to_vec(&Stakes::new()).unwrap());
            deps.storage.set(RESULT_KEY, &serde_json::to_vec(&Results::new()).unwrap());

            Ok(InitResponse::default())
        }
    }
}
pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Ruler {phrase, prediction_number, position, bet_amount} => try_ruler(
            deps, 
            env,
            phrase,
            prediction_number,
            position,
            &bet_amount,
        ),
        HandleMsg::TryCapitalDeposit{} => try_capital_deposit(
            deps, 
            env,
        ),
        HandleMsg::TryCapitalWithdraw{} => try_capital_withdraw(
            deps, 
            env,
        ),
    }
}
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCasinoStakeInfo {} => to_binary(
            &read_casino_stake_info(
                deps
            )?
        ),
        QueryMsg::GetStakeInfo {} => to_binary(
            &read_stake_info(
                deps
            )?
        ),
        QueryMsg::GetCasinoInfo {} => to_binary(
            &read_casino_info(
                deps
            )?
        ),
        QueryMsg::Getmystate{address}=> to_binary(
            &read_room_state(
                &address,
                &deps.storage,
                &deps.api
            )?
        )
    }
}

fn try_capital_deposit<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut amount_raw: Uint128 = Uint128::default();

    for coin in &env.message.sent_funds {
        if coin.denom == "uscrt" {
            amount_raw = coin.amount
        }
    }

    if amount_raw == Uint128::default() {
        return Err(StdError::generic_err(format!("Lol send some funds dude")));
    }

    let mut casino: Casino = serde_json::from_slice(&deps.storage.get(CASINO_KEY).unwrap()).unwrap();
    let stake: Stakes = serde_json::from_slice(&deps.storage.get(STAKE_KEY).unwrap()).unwrap();

    let mut new_stakes = Stakes::new();
    let new_capital = casino.capital + amount_raw;
    
    for stake_info in stake.iter() {
        if env.message.sender == stake_info.address {
            return Err(StdError::generic_err(format!("already deposit")));
        }
        let deposit = casino.capital.u128() * stake_info.ownership_percentage.u128() / 100000000;
        let new_ownership_percentage = Uint128(100000000 * deposit / new_capital.u128());
        let mut new_stake_info:StakeInfo = stake_info.clone();
        new_stake_info.ownership_percentage = new_ownership_percentage;
        new_stakes.push(new_stake_info.clone());
    }

    let my_stake_info: StakeInfo = StakeInfo{
        address: env.message.sender,
        begin_amount: amount_raw,
        ownership_percentage: Uint128(100000000 * amount_raw.u128() / new_capital.u128()),
    };

    new_stakes.push(my_stake_info.clone());

    casino.capital = new_capital;

    deps.storage.set(CASINO_KEY, &serde_json::to_vec(&casino).unwrap());
    deps.storage.set(STAKE_KEY, &serde_json::to_vec(&new_stakes).unwrap());
    Ok(HandleResponse::default())
}

fn try_capital_withdraw<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut casino: Casino = serde_json::from_slice(&deps.storage.get(CASINO_KEY).unwrap()).unwrap();
    let stake: Stakes = serde_json::from_slice(&deps.storage.get(STAKE_KEY).unwrap()).unwrap();

    let mut amount_raw: u128 = 0;

    for stake_info in stake.iter() {
        if env.message.sender == stake_info.address {
            amount_raw = stake_info.ownership_percentage.u128()*casino.capital.u128()/100000000;
        }
    }

    if amount_raw == 0 {
        return Err(StdError::generic_err(format!("not found deposit!")));
    }

    let mut new_stakes = Stakes::new();
    let new_capital = casino.capital.u128() - amount_raw;

    for stake_info in stake.iter() {
        if env.message.sender != stake_info.address {
            let deposit = casino.capital.u128() * stake_info.ownership_percentage.u128() / 100000000;
            let new_ownership_percentage = Uint128(100000000 * deposit / new_capital);
            let mut new_stake_info:StakeInfo = stake_info.clone();
            new_stake_info.ownership_percentage = new_ownership_percentage;
            new_stakes.push(new_stake_info.clone());
        }
    }

    casino.capital = Uint128(new_capital);

    deps.storage.set(CASINO_KEY, &serde_json::to_vec(&casino).unwrap());
    deps.storage.set(STAKE_KEY, &serde_json::to_vec(&new_stakes).unwrap());

    let transfer = send_coin(&env, Uint128(amount_raw)).unwrap();
    let res = HandleResponse {
        messages: vec![transfer],
        log: vec![
            log("action", "capital_withdraw"),
        ],
        data: None,
    };
    Ok(res)
}

pub fn send_coin(
    env : &Env,
    payout_amount: Uint128,
)-> StdResult<CosmosMsg> {
    let token_transfer = BankMsg::Send {
        from_address: env.contract.address.clone(),
        to_address: env.message.sender.clone(),
        amount: vec![Coin {
            denom: "uscrt".to_string(),
            amount: payout_amount,
        }],
    }
    .into();
    Ok(token_transfer)
}

pub fn can_winer_payout(
    env : &Env,
    amount: Uint128,
)-> StdResult<HandleResponse> {
    let token_transfer = BankMsg::Send {
        from_address: env.contract.address.clone(),
        to_address: env.message.sender.clone(),
        amount: vec![Coin {
            denom: "uscrt".to_string(),
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
    // uscrt =1000000 = 1krw 
    //98.5/Prediction=multiplier

    match &position[..] {
        "over" => {
            multiplier = (1000000 as u128- fee as u128)/(99 as u128-(prediction_number as u128*5/3));
            let bet_amount = *bet_amount;
            payout = bet_amount.u128() * multiplier/10000;
        },
        _ => {
            multiplier = (1000000 as u128- fee as u128)/(prediction_number as u128*5/3);
            let bet_amount = *bet_amount;
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
    if &position[..] == "over"{
        if prediction_number < 2 || prediction_number > 58 {
            return Err(StdError::generic_err(
                "prediction number, 2~58",
            ));
        }
    }

    if &position[..] == "under"{
        if prediction_number < 1 || prediction_number > 57 {
            return Err(StdError::generic_err(
                "prediction number, 1~57",
            ));
        }
    }

    let mut casino: Casino = serde_json::from_slice(&deps.storage.get(CASINO_KEY).unwrap()).unwrap();
    
    //3.prediction check is pool amount check
    
    let payout = payout_amount(
        prediction_number,
        position.clone(), 
        bet_amount,
        casino.house_fee
    )?;

    if casino.capital.u128() * casino.max_bet_rate as u128 / 1000000 < payout {
        return Err(StdError::generic_err(format!("Lack of reserves capital={}, payout={}, max-bet-rate={}",casino.capital, payout, casino.max_bet_rate)));
    }
    
    //4. user demon/amount check - Users should also double check
    //Minimum bet / maximum bet limit
    let mut amount_raw: Uint128 = Uint128::default();
    for coin in &env.message.sent_funds {
        if coin.denom == "uscrt" {
            amount_raw = coin.amount
        } else{
            return Err(StdError::generic_err(format!(
                "Insufficient uscrt denom",
            )));
        }
    }
    if amount_raw != *bet_amount {
        return Err(StdError::generic_err(format!(
            "Insufficient uscrt deposit: bet_amount={}, required={}",
            *bet_amount, amount_raw
        )));
    } else if env.message.sent_funds.len() == 0{
        return Err(StdError::generic_err("SHOW ME THE MONEY"));
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
    let mut rng: Prng = Prng::new(&casino.seed, &rand_entropy);

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
        results: win_results,
        payout: Uint128(payout),
        bet_amount: *bet_amount,
    })?;
    let mut room_store = PrefixedStorage::new(ROOM_KEY, &mut deps.storage);
    room_store.set(raw_address.as_slice(), &raw_room); 

    //10. Distribution of rewards by win and lose
    //let contract_address_raw = deps.api.human_address(&env.contract.address)?;
    //let recipient_address_raw = deps.api.human_address(&env.message.sender)?;
    casino.bet_cumulative_amount += *bet_amount;
    if win_results == false {
        casino.capital += amount_raw;
        deps.storage.set(CASINO_KEY, &serde_json::to_vec(&casino).unwrap());
        return Ok(HandleResponse::default());
    } else {
        casino.capital = Uint128(casino.capital.u128() + amount_raw.u128() - payout);
        let _ = can_winer_payout(&env, Uint128::from(payout as u128));
        let send_result : HandleResponse = can_winer_payout(&env, Uint128::from(payout as u128)).unwrap();
        
        deps.storage.set(CASINO_KEY, &serde_json::to_vec(&casino).unwrap());
        return Ok(send_result);
    }
}

fn read_casino_stake_info<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> StdResult<CasinoStakeResponse> {
    let casino: Casino = serde_json::from_slice(&deps.storage.get(CASINO_KEY).unwrap()).unwrap();
    let stake: Stakes = serde_json::from_slice(&deps.storage.get(STAKE_KEY).unwrap()).unwrap();
    Ok(CasinoStakeResponse{
        casino,
        stake,
    })
}


fn read_stake_info<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> StdResult<StakeResponse> {
    let stake: Stakes = serde_json::from_slice(&deps.storage.get(STAKE_KEY).unwrap()).unwrap();
    Ok(StakeResponse{
        stake,
    })
}

fn read_casino_info<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>
) -> StdResult<CasinoResponse> {
    let casino: Casino = serde_json::from_slice(&deps.storage.get(CASINO_KEY).unwrap()).unwrap();
    Ok(CasinoResponse{
        casino,
    })
}

fn read_room_state<S: Storage, A: Api>(
    address: &HumanAddr,
    store: &S,
    api: &A,
) -> StdResult<RoomStateResponse> {
    let owner_address = api.canonical_address(address)?;
    let room_store = ReadonlyPrefixedStorage::new(ROOM_KEY, store);
    let room_state = room_store.get(owner_address.as_slice()).unwrap();
    let room : Room = from_slice(&room_state).unwrap();
    let payout = room.payout.u128();
    let amount = room.bet_amount.u128();
    Ok(RoomStateResponse{
        start_time: room.start_time,
        entropy: room.entropy,
        prediction_number: room.prediction_number,
        lucky_number: room.lucky_number,
        position: room.position,
        results: room.results,
        payout: payout as u64,
        bet_amount: amount as u64,
    })

}