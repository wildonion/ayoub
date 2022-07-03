#!/bin/bash
near login
echo "[?] Near Network >>>"
read NETWORK
echo "[?] Account ID - Logged In Account; With .testnet >>>"
read OWNER_ID # NOTE - the account id to (re)deploy the contract on which is the owner or the signer of the contract
NEAR_ENV=$NETWORK near deploy --wasmFile out/market.wasm --accountId $OWNER_ID