casper-client put-deploy \
--node-address http://188.40.47.161:7777/rpc \
--chain-name casper-test \
--secret-key ./keys/secret_key.pem \
--payment-amount 1000000000 \
--session-path ./multisig/client/target/wasm32-unknown-unknown/release/contract.wasm \
--session-arg "contract_hash:key='hash-4e2a6efaf0c8dea9df6c0cd0f055afc309ee3c399cd4ee1bdd7ece528277f893'" \
--session-arg "amount:u512='10000000000'"
