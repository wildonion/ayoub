#!/bin/bash



near login # NTOE - use this to login with your account id to set for caller id
echo "[?] Your Account ID For Calling Contract Methods - Logged In Account >>>"
read CALLER_ID
echo "[?] Contract Account ID >>>"
read CONTRACT_ACCOUNT_ID
echo "[?] Receiver Account ID - Mint NFT On? >>>"
read RECEIVER_ID # NOTE - the owner account_id that the NFT must be minted on  
echo "[?] Contract Owner Account ID - Contract Account ID or Another Account ID >>>"
read CONTRACT_OWNER_ID # NOTE - it can be the contract id itself or another account id or even it could be empty at all
echo "[?] Fetch NFT for Which Account ID >>>"
read NFT_FOR_OWNER_ID
near call $CONTRACT_ACCOUNT_ID new_default_meta '{"owner_id": "'$CONTRACT_OWNER_ID'"}' --accountId $CALLER_ID # NOTE - first of first we have to initialize the contract; i'll be paniced on second call
near call $CONTRACT_ACCOUNT_ID nft_mint '{"token_id": "90890908908906", "metadata": {"title": "A Gift!", "description": "WoW :)", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "'$RECEIVER_ID'"}' --accountId $CALLER_ID --amount 0.1 # NOTE - it'll be paniced on second call with NFT already minted message 
near view $CONTRACT_ACCOUNT_ID nft_metadata
near view $CONTRACT_ACCOUNT_ID nft_token '{"token_id": "token-2"}'
near view $CONTRACT_ACCOUNT_ID nft_tokens_for_owner '{"account_id": "'$NFT_FOR_OWNER_ID'", "limit": 5}'
near view $CONTRACT_ACCOUNT_ID nft_tokens '{"from_index": "10", "limit": 50}'
near view $CONTRACT_ACCOUNT_ID nft_supply_for_owner '{"account_id": "'$NFT_FOR_OWNER_ID'"}'
near call $CONTRACT_ACCOUNT_ID nft_transfer '{"receiver_id": "'$RECEIVER_ID'", "token_id": "token-2", "memo": "Begiresh :)"}' --accountId $CALLER_ID --depositYocto 1
near call $CONTRACT_ACCOUNT_ID nft_mint '{"token_id": "token-3", "metadata": {"title": "New NFT Minted", "description": "Testing the transfer call function", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "'$RECEIVER_ID'"}' --accountId $CALLER_ID --amount 0.1
near call $CONTRACT_ACCOUNT_ID nft_transfer_call '{"receiver_id": "wildonion.testnet", "token_id": "token-1", "msg": "not exist account :)"}' --accountId $CALLER_ID --depositYocto 1 --gas 200000000000000 # NOTE - if you try to transfer an NFT to a receiver that does not implement the nft_on_transfer function or doesn't have a contract, the contract will panic and the NFT will not be transferred thus NFT should remain yours after the transaction is complete








if [ -z "$SUB_MASTER_CONTRACT_ID" ] # NOTE - check it from the env var which has been set when we were deploying the contract  
then
    echo "[?] No Sub Master Account Detected!"
else
    echo "[?] Receiver Account ID - Mint NFT On? >>>"
    read RECEIVER_ID # NOTE - the owner account_id that the NFT must be minted on
    near call $SUB_MASTER_CONTRACT_ID new_default_meta '{"owner_id": "'$SUB_MASTER_CONTRACT_ID'"}' --accountId $SUB_MASTER_CONTRACT_ID # NOTE - first of first we have to initialize the contract; i'll be paniced on second call - we've used the SUB_MASTER_CONTRACT_ID itself as the caller of this method
    near call $SUB_MASTER_CONTRACT_ID nft_mint '{"token_id": "approval-token", "metadata": {"title": "Approval Token", "description": "testing out the new approval extension of the standard", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "'$RECEIVER_ID'"}' --accountId $SUB_MASTER_CONTRACT_ID --amount 0.1 # NOTE - next_approval_id field of the token object will be 0 with an empty hashmap or {} for its approved_account_ids field
    near view $SUB_MASTER_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$SUB_MASTER_CONTRACT_ID'", "limit": 10}' # NOTE - approved_account_ids field from the token object must be an empty hashmap or {} if there was no approved account_id inside of it to transfer the NFT on behalf of the owner
    near call $SUB_MASTER_CONTRACT_ID nft_approve '{"token_id": "approval-token", "account_id": "'$CONTRACT_ACCOUNT_ID'"}' --accountId $SUB_MASTER_CONTRACT_ID --deposit 0.1 # NOTE - approving the master contract actor account_id or CONTRACT_ACCOUNT_ID; we don't need to pass a message since the CONTRACT_ACCOUNT_ID account didn't implement the nft_on_approve() method thus we can't schedule a cross contract call promise method also the caller of this method must attach enough NEAR to cover the cost of storing the account on the contract, 0.1 NEAR should be more than enough and the caller will be refunded any excess that is unused
    near view $SUB_MASTER_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$SUB_MASTER_CONTRACT_ID'", "limit": 10}' # NOTE - we must see the new approved account_id being returned which is the CONTRACT_ACCOUNT_ID account_id - now we can see that the next_approval_id field of the token object has increased by 1
    near call $SUB_MASTER_CONTRACT_ID nft_transfer '{"receiver_id": "'$CONTRACT_ACCOUNT_ID'", "token_id": "approval-token", "approval_id": 1}' --accountId $CONTRACT_ACCOUNT_ID --depositYocto 1 # NOTE - ********* THIS WOULD CAUSE TO A PANIC CAUSE THERE IS NO SUCH APPROVAL ID WITH VALUE 1 *********
    near call $SUB_MASTER_CONTRACT_ID nft_transfer '{"receiver_id": "'$CONTRACT_ACCOUNT_ID'", "token_id": "approval-token", "approval_id": 0}' --accountId $CONTRACT_ACCOUNT_ID --depositYocto 1 # NOTE - CONTRACT_ACCOUNT_ID can now transfer an NFT on behalf of the owner which is the SUB_MASTER_CONTRACT_ID to him/her-self 
    near view $SUB_MASTER_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$SUB_MASTER_CONTRACT_ID'", "limit": 10}' # NOTE - we should see that the token info has been update including the owner_id and the reset approved_account_ids
    near call $SUB_MASTER_CONTRACT_ID nft_approve '{"token_id": "approval-token", "account_id": "'$SUB_MASTER_CONTRACT_ID'"}' --accountId $CONTRACT_ACCOUNT_ID --deposit 0.1 # NOTE - approving the sub master contract actor account_id or SUB_MASTER_CONTRACT_ID using the master account_id which is the CONTRACT_ACCOUNT_ID; we don't need to pass a message since the SUB_MASTER_CONTRACT_ID account didn't implement the nft_on_approve() method thus we can't schedule a cross contract call promise method also the caller of this method must attach enough NEAR to cover the cost of storing the account on the contract, 0.1 NEAR should be more than enough and the caller will be refunded any excess that is unused
    near view $SUB_MASTER_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$CONTRACT_ACCOUNT_ID'", "limit": 10}' # NOTE - getting the token info for master account_id which is the CONTRACT_ACCOUNT_ID cause we transferred the NFT earlier to CONTRACT_ACCOUNT_ID and we should see that approved_account_ids field has filled up with an approved account_id which is the sub master account_id or SUB_MASTER_CONTRACT_ID who has minted this NFT also its approval_id is 1 and the next_approval_id is 1 means that the next caller for transferring this NFT behalf of its owner is the sub master account itself which is SUB_MASTER_CONTRACT_ID since we didn't approve any other account for this NFT
    near call $SUB_MASTER_CONTRACT_ID nft_approve '{"token_id": "approval-token", "account_id": "smartie"}' --accountId $CONTRACT_ACCOUNT_ID --deposit 0.1 # NOTE - approving the smartie contract actor account_id to transfer the NFT on behalf of its owner which is CONTRACT_ACCOUNT_ID or the master account_id; in this stage the approved_account_ids field contains two approved accounts (SUB_MASTER_CONTRACT_ID and smartie contract actor account_id) and the next_approval_id will be 2 means the first approved account_id can't transfer the NFT on behalf of its owner anymore 
    near view $SUB_MASTER_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$SUB_MASTER_CONTRACT_ID'", "limit": 10}'
    near call $SUB_MASTER_CONTRACT_ID nft_mint '{"token_id": "another-token", "metadata": {"title": "Approval Token", "description": "testing out the new approval extension of the standard", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "'$RECEIVER_ID'", "perpetual_royalties": {"benjiman.testnet": 2000, "mike.testnet": 1000, "josh.testnet": 500}}' --accountId $SUB_MASTER_CONTRACT_ID --amount 0.1 # NOTE - minting NFT with perpetual royalty param
    near view $SUB_MASTER_CONTRACT_ID nft_payout '{"token_id": "approval-token", "balance": "100", "max_len_payout": 100}' # NOTE - paying out 100 owners
    near view $SUB_MASTER_CONTRACT_ID nft_transfer_payout '{"receiver_id": "'$RECEIVER_ID'", "token_id": "approval-token", "approval_id": 1, "balance": "100", "max_len_payout": 100}' --accountId $SUB_MASTER_CONTRACT_ID --depositYocto 1 # NOTE - transfer an NFT to a receiver_id's contract actor then payout 100 owners
fi