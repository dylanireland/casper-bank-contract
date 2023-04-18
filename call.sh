casper-client put-deploy \
--node-address http://188.40.47.161:7777/rpc \
--chain-name casper-test \
--secret-key ./keys/secret_key.pem \
--payment-amount 1000000000 \
--session-hash hash-237ad07c4f27c78d13cad51c2f94f4a74633aa9189a4ca38dd6c8a1dfe91cb16 \
--session-entry-point "deposit" \
--session-arg "amount:u512='10000000000'" \
