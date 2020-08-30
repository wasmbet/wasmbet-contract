# Wasmbet


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
secretcli tx compute store contract.wasm.gz --from wasmbet_contract_dice.wasm --gas auto -y
secretcli query compute list-code
secretcli tx compute instantiate 140 --label wasmbet '{"seed":"allinbiteqwe","min_credit":"1000000","max_credit":"10000000","house_fee":1500}' --from wasmbetv
secretcli tx compute execute secret1yug7wrsjyeufpfghuhwfzef0hqrmyju9kftzgl '{"try_pot_pool_deposit":{}}' 100000000uscrt --from wasmbetv
secretcli q compute query secret1yug7wrsjyeufpfghuhwfzef0hqrmyju9kftzgl '{"getstate":{}}' 
secretcli tx compute execute secret1yug7wrsjyeufpfghuhwfzef0hqrmyju9kftzgl '{"ruler":{"phrase":"allinbitewjkrwerlwerwerbfcwl","prediction_number":50,"position":"under","bet_amount":"1000000"}}' --amount 1000000uscrt --from wasmbetv
secretcli q compute query secret1yug7wrsjyeufpfghuhwfzef0hqrmyju9kftzgl '{"getmystate":{"address":"secret1jzrfydf9a0v4ame8feh33k9en7mklmh9u9p30l"}}' 
```