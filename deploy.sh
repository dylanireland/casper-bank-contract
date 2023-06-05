casper-client put-deploy \
--node-address http://188.40.47.161:7777/rpc \
--chain-name casper-test \
--secret-key ./keys/secret_key.pem \
--payment-amount 30000000000 \
--session-path ./contract/contract/target/wasm32-unknown-unknown/release/contract.wasm
