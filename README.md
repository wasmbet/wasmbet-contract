# Wasmbet


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --lock
terracli tx wasm store target/wasm32-unknown-unknown/release/wasmbet_contract_dice.wasm --from validator --chain-id tequila-0002 --gas 2000000 --node tcp://127.0.0.1:16657
terracli q tx <txhash> --trust-node --node tcp://127.0.0.1:16657 | grep code_id
terracli  tx wasm instantiate 40 '{"seed":"<rand seed>","min_credit":"<min>","max_credit":"<max>","house_fee":<fee>}' --from validator --node tcp://127.0.0.1:16657 --chain-id tequila-0002
terracli q tx <txhash> --trust-node --node tcp://127.0.0.1:16657 | grep contract_address
terracli tx wasm execute <contract_address> '{"try_pot_pool_deposit":{}}' 100000000ukrw --from validator --node tcp://127.0.0.1:16657 --chain-id tequila-0002
terracli tx wasm execute <contract_address>'{"ruler":{"phrase":"allinbitewjkrwerlwerwerbfcwl","prediction_number":50,"position":"under","bet_amount":"1000000"}}' 1000000ukrw --from validator --node tcp://127.0.0.1:16657 --chain-id tequila-0002
terracli q wasm contract-store <contract_address> '{"getstate":{}}' --node tcp://127.0.0.1:16657 --chain-id tequila-0002
terracli q wasm contract-store <contract_address> '{"getmystate":{"address":"<bet address>"}}' --node tcp://127.0.0.1:16657 --chain-id tequila-0002 
```