#!/bin/bash
set -e

ID=murkwoodtale1.sergantche.testnet

# recreate account
# near delete $ID sergantche.testnet
near create-account $ID --masterAccount=sergantche.testnet --initial-balance 50

# deploy contract
near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/near_backend.wasm --accountId $ID
near call $ID new --accountId $ID