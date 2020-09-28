# Wasmbet


```sh
RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
terracli tx wasm store target/wasm32-unknown-unknown/release/wasmbet_contract_timeroulette.wasm --from teste --gas 2000000 --gas-prices 178.05ukrw --chain-id tequila-0004 --node tcp://127.0.0.1:26657
terracli q tx 51AB2B71F32CA885BEF6250A58DA38D2284727DB3D5790BCFCE561907708C527 --trust-node --node tcp://127.0.0.1:26657 | grep code_id
terracli tx wasm instantiate 119 '{"seed":"<rand seed>","min_amount":"1000000","max_amount":"100000000","house_fee":15000}' --from teste --gas auto --gas-prices 178.05ukrw --node tcp://127.0.0.1:26657 --chain-id tequila-0004
terracli q tx <txhash> --trust-node --node tcp://127.0.0.1:26657 | grep contract_address
terracli tx wasm execute terra1dmndhy4jd3ja56d6082np4sldddphstjzcjzw8 '{"try_pot_pool_deposit":{}}' 100000000ukrw --from teste --gas 130000 --gas-prices 178.05ukrw --node tcp://127.0.0.1:26657 --chain-id tequila-0004 
terracli q wasm contract-store terra1dmndhy4jd3ja56d6082np4sldddphstjzcjzw8 '{"getstate":{}}' --node tcp://127.0.0.1:26657 --chain-id tequila-0004
terracli tx wasm execute terra1dmndhy4jd3ja56d6082np4sldddphstjzcjzw8 '{"ruler":{"phrase":"allinbitewjkrwerlwerwerbfcwl","prediction_number":50,"position":"under","bet_amount":"2000000"}}' 2000000ukrw --from teste --gas 150000 --gas-prices 178.05ukrw --node tcp://127.0.0.1:26657 --chain-id tequila-0004
terracli q wasm contract-store terra1dmndhy4jd3ja56d6082np4sldddphstjzcjzw8 '{"getmystate":{"address":"terra188ugd397qpdu7qr2jyqw5mm08x7g2nyn7redn4"}}' --node tcp://127.0.0.1:26657 --chain-id tequila-0004
```
