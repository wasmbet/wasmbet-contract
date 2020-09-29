# simple explanation

Currently, Kpler doesn't support testnets, so it's not smooth, but the rule is this.

User

1. It is possible to create your own casino

2. It is possible to stake in another pool without creating a casino

3. You can only gamble without creating a casino or staking.

4. Gambling is possible in the pool the user wants. Probably, users will be driven to places where house fees are low, and liquidity pools will compete for fees.

Also, creating or staking a casino means,

It's like playing a reverse betting game with a gambling user,

If the user wins, the pool reports a deficit
When defeated, the pool benefits.

Each holder in the pool can earn profits by stake.

This is the same as the casino principle.

The larger the pool and the cheaper the fee, the longer the profit can be.

Later, through the IBC, the liquidity pool can be combined in all wasm support zones,

A huge casino where users can participate in casino profits

It is a similar decentralized financial gambling market.

For reference, the Founder Fee is the fee that the creator of the casino receives from the stakers, and we are considering whether to add or subtract.(
Necessary for motivation to create a casino)



# Wasmbet


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
secretcli tx compute store ./target/wasm32-unknown-unknown/release/wasmbet_contract_timeroulette.wasm --from test --gas 2000000 -y --chain-id test
secretcli tx compute store wasmbet_contract_timeroulette.wasm --from test --gas 2000000 -y --chain-id test
secretcli query compute list-code

secretcli tx compute instantiate 14 --label wasmbet_casinoname141 '{"CreateCasino": {"name": "F1 Casino", "description": "come on rich","seed":"allinbiteqwe","min_bet_amount":"1000000","max_bet_rate":100000,"house_fee":15000, "founder_commission_rate": 100000}}' --from test --chain-id test


secretcli tx compute instantiate 7 --label wasmbet_casinonamez '{"CreateCasino": {"name": "F1 Casino", "min_bet_amount":"1000000" ,"description": "come on ric", "z": "z"}}' --from test --chain-id test

secretcli q compute list-contract-by-code 11

secretcli tx send test secret1cwgm3mj4scd8s4dacj35kd749cr3lew02twtlr 100000000000uscrt --chain-id test

secretcli query tx 9EC7F490A077942A0E84EAC8F59D325B13805F624A389AA12D141DFB9A98C214 | grep log
secretcli query compute tx DF3293DED02AFC790DA01535AE6AFCF6A597A73CB14A66A58D798438BDB3D31B

secretcli tx compute instantiate 1 --label wasmbet_casinoname '{"seed":"allinbiteqwe","min_credit":"1000000","max_credit":"10000000","house_fee":1500}' --from test --chain-id test
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"try_pot_pool_deposit":{}}' --amount 100000000uscrt --from test
secretcli q compute query secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"getstate":{}}' 
secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"ruler":{"phrase":"allinbitewjkrwerlwerwerbfcwl","prediction_number":50,"position":"under","bet_amount":"1000000"}}' --amount 1000000uscrt --from wasmbetv
secretcli q compute query secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg '{"getmystate":{"address":"secret1jzrfydf9a0v4ame8feh33k9en7mklmh9u9p30l"}}' 
```


