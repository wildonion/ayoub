#!/bin/bash



near login
echo "[?] Near Network >>>"
read NETWORK




echo "--------------------- NEAR MASTER ACCOUNT DEPLOYMENT ---------------------"
echo "[?] Master Account ID - Logged In Account; With .testnet >>>"
read OWNER_ID # NOTE - the account id to (re)deploy the contract on which is the owner or the signer of the contract
NEAR_ENV=$NETWORK near deploy --wasmFile out/nft.wasm --accountId $OWNER_ID




echo "--------------------- NEAR SUB MASTER ACCOUNT DEPLOYMENT ---------------------"
echo "[?] Sub Master Account ID; Without .testnet >>>"
read SUB_MASTER_CONTRACT_ID
if [ -z "$SUB_MASTER_CONTRACT_ID" ]
then
    echo "[?] No Sub Master Account Entered!"
else
    echo "[...] Deploying on Sub Master Account"
    near create-account $SUB_MASTER_CONTRACT_ID.$OWNER_ID --masterAccount $OWNER_ID --initialBalance 25
    NEAR_ENV=$NETWORK near deploy --wasmFile out/nft.wasm --accountId $SUB_MASTER_CONTRACT_ID.$OWNER_ID
fi