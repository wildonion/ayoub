



// viewing nft metadata and the token itself methods of the deployed bluerangene contract
// ...
// near view $NFT_CONTRACT_ID nft_metadata
// near view $NFT_CONTRACT_ID nft_token '{"token_id": "token-1"}'
// near view $NFT_CONTRACT_ID nft_tokens_for_owner '{"account_id": "'$NFT_CONTRACT_ID'", "limit": 5}'