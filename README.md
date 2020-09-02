# Wasmbet


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --lock
secretcli tx compute store wasmbet_contract_timeroulette.wasm --from <wallet name> --gas auto -y
secretcli query compute list-code
secretcli tx compute instantiate <codeid> --label wasmbet '{"seed":"<rand string>","min_amount":"1000000","max_amount":"10000000","house_fee":1500}' --from <wallet name>
secretcli tx compute execute secret1q7d3msna9an4eyvhm6ttur8jc3f4z38h5dtjyz '{"try_pot_pool_deposit":{}}' --amount 100000000uscrt --from wasmbetv
secretcli q compute query secret1q7d3msna9an4eyvhm6ttur8jc3f4z38h5dtjyz '{"getstate":{}}' 
secretcli tx compute execute secret1q7d3msna9an4eyvhm6ttur8jc3f4z38h5dtjyz '{"ruler":{"phrase":"<rand string>","prediction_number":50,"position":"under","bet_amount":"1000000"}}' --amount 1000000uscrt --from <wallet name>
secretcli q compute query <Contract address> '{"getmystate":{"address":"<you account address>"}}' 
```