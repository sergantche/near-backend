#!/bin/bash
set -e

CONTRACT=heroes_v1.sergantche.testnet
MASTER_ACCOUNT=sergantche.testnet

# create subaccount
# near delete $CONTRACT $MASTER_ACCOUNT # uncomment to delete old account
near create-account $CONTRACT --masterAccount=$MASTER_ACCOUNT --initial-balance 50

# deploy contract
near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/near_backend.wasm --accountId $CONTRACT
near call $CONTRACT new --accountId $CONTRACT

# copy credentials for later deploy
cp ~/.near-credentials/testnet/$CONTRACT.json ./creds