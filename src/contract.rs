use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,CanonicalAddr
};

use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State, Room_State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let room = Room_State {
        room_owner: CanonicalAddr::default(),
        seed : Vec::default(),
        start_time: env.block.time,
        entropy: Vec::default(),
        Prediction_number: 0,
        Lucky_Number: 0,
        position: String::default(),
        results: String::default(),
    };
    let state = State {
        contract_owner: deps.api.canonical_address(&env.message.sender)?,
        player_room: Vec::default(),
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
        HandleMsg::Ruler {player,seed, Prediction_number, position} => try_ruler(deps, env),
    }
}

pub fn try_ruler<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
) -> StdResult<HandleResponse> {
    let mut state = config(&mut deps.storage).load()?;
    let mut game_room = state.player_room;
    game_room.seed

    //rand
    //deposit
    //Distribution of prizes based on results


    config(&mut deps.storage).update(|mut state| {
        state.count += 1;
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn try_reset<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    count: i32,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    config(&mut deps.storage).update(|mut state| {
        if api.canonical_address(&env.message.sender)? != state.owner {
            return Err(StdError::unauthorized());
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

fn rand<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    winner_to_select: u8,
) -> StdResult<HandleResponse> {
    // TODO Check if contract has expired

    let mut state = config(&mut deps.storage).load()?;

    // add this if you don't want to allow choosing an alternative winner
    // if state.winner != CanonicalAddr::default() {
    //     // game already ended
    //     return Ok(HandleResponse::default());
    // }

    if env.message.sender != state.contract_owner {
        return Err(throw_gen_err("You cannot trigger lottery end unless you're the owner!".to_string()));
    }
    // let contract_addr: HumanAddr = deps.api.human_address(&env.contract.address)?;

    // this way every time we call the end_lottery function we will get a different result. Plus it's going to be pretty hard to
    // predict the exact time of the block, so less chance of cheating
    state.entropy.extend_from_slice(&env.block.time.to_be_bytes());

    let mut rng: Prng = Prng::new(&state.seed, &state.entropy);

    let winner = rng.select_one_of(state.items.clone().into_iter());

    if winner.is_none() {
        return Err(throw_gen_err(format!("Fucking address is empty wtf")));
    }

    let unwrapped = winner.unwrap();

    match winner_to_select {
        1 => {
            state.winner1 =  (&unwrapped).clone();
        },
        2 => {
            state.winner2 =  (&unwrapped).clone();
        },
        3 => {
            state.winner3 =  (&unwrapped).clone();
        },
        _ => {
            return Err(throw_gen_err(format!("bad winner selection")));
        }
    }

    config(&mut deps.storage).save(&state)?;

    let winner_readable = deps.api.human_address(&unwrapped)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("winner", format!("{}", winner_readable))],
        data: None,
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CountResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(CountResponse { count: state.count })
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
