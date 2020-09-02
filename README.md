# Wasmbet


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
secretcli tx compute store wasmbet_contract_timeroulette.wasm --from <wallet name> --gas 1100000
secretcli query compute list-code
secretcli tx compute instantiate <codeid> --label wasmbet '{"seed":"<rand string>","min_amount":"1000000","max_amount":"10000000","house_fee":1500}' --from <wallet name>
secretcli tx compute execute <contract address> '{"try_pot_pool_deposit":{}}' --amount 100000000uscrt --from <wallet name>
secretcli q compute query <contract address> '{"getstate":{}}' 
secretcli tx compute execute <contract address> '{"ruler":{"phrase":"<rand string>","prediction_number":50,"position":"under","bet_amount":"1000000"}}' --amount 1000000uscrt --from <wallet name>
secretcli q compute query <contract address> '{"getmystate":{"address":"<you account address>"}}' 
```