



// viewing nft metadata and the token itself methods of the deployed contract
// ...
// near view $NFT_CONTRACT_ID nft_token '{"token_id": "token-1"}'
// near view $NFT_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$NFT_CONTRACT_ID'", "limit": 5}'
// near view $NFT_CONTRACT_ID nft_tokens '{"from_index": "10", "limit": 50}'
// near view $NFT_CONTRACT_ID nft_supply_for_owner '{"account_id": "goteam.testnet"}'