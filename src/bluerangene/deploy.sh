#!/bin/bash
echo "[?] Your Account ID >>>"
read ID
echo "[?] Near Network >>>"
read NETWORK
echo "account" $ID
echo "network" $NETWORK
NEAR_ENV=$NETWORK
ID=$ID
near login
NEAR_ENV=$NETWORK near deploy --wasmFile out/bluerangene.wasm --accountId $ID