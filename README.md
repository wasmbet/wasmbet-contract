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
