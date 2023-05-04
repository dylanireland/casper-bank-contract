casper-client put-deploy \
--node-address http://188.40.47.161:7777/rpc \
--chain-name casper-test \
--secret-key ./keys/secret_key.pem \
--payment-amount 10000000000 \
--session-path ./multisig/client/target/wasm32-unknown-unknown/release/deposit.wasm \
--session-arg "contract_hash:key='hash-9dabfd25257e9372363685440b0b14de20ca0ea1e4f8eb7250f7416691c5b30e'" \
--session-arg "amount:u512='10000000000'"
