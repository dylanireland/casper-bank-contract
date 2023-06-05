casper-client put-deploy \
--node-address http://188.40.47.161:7777/rpc \
--chain-name casper-test \
--secret-key ./keys/secret_key.pem \
--payment-amount 10000000000 \
--session-path ./contract/client/target/wasm32-unknown-unknown/release/deposit.wasm \
--session-arg "contract_hash:key='hash-b62481085812c4f56027f5fb7f3e5e2f087b14190da8762fb88889e4945178be'" \
--session-arg "amount:u512='100000000'"
