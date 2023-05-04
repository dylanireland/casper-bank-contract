if ! command -v jq >/dev/null 2>&1; then
    brew install jq
fi
srh_json=$(casper-client get-state-root-hash --node-address http://188.40.47.161:7777/rpc)
srh=$(echo "$srh_json" | jq -r '.result.state_root_hash')

casper-client query-global-state \
--node-address http://188.40.47.161:7777/rpc \
--state-root-hash $srh \
--key hash-4f4d3cb768e034cfbfb86e7d0e41c7e6f642085576d221671fa5de94a8184a69 \
-q "balances/35eff51c55323b393d5c65b3111af22ca977b9fac798f89b8f5703d6d273616e"

