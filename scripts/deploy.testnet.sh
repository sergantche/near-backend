#!/bin/bash
set -e

ID=hackerman3.sergantche.testnet

# recreate account
near delete $ID sergantche.testnet
near create-account $ID --masterAccount=sergantche.testnet --initial-balance 50

# deploy contract
near deploy --wasmFile out/main.wasm --accountId $ID
near call $ID new --accountId $ID