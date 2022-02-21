#!/bin/bash
set -e

ID=heroes_v1.sergantche.testnet

# create subaccount
# near delete $ID sergantche.testnet # uncomment to delete old account
near create-account $ID --masterAccount=sergantche.testnet --initial-balance 50

# deploy contract
near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/near_backend.wasm --accountId $ID
near call $ID new --accountId $ID

# copy credentials for later deploy
cp ~/.near-credentials/testnet/$ID.json ./creds