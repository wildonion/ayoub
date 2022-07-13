#!/bin/bash





near login # NTOE - use this to login with your account id to set for caller id
echo "[?] Market Contract Account ID - Logged In Account >>>"
read MARKET_CONTRACT_ACTOR_ACCOUNT_ID
near call $MARKET_CONTRACT_ACTOR_ACCOUNT_ID new '{"owner_id": "'$MARKET_CONTRACT_ACTOR_ACCOUNT_ID'"}' --accountId $MARKET_CONTRACT_ACTOR_ACCOUNT_ID # NOTE - first of first we have to initialize the contract; i'll be paniced on second call
near view $MARKET_CONTRACT_ACTOR_ACCOUNT_ID get_supply_sales # query for the total supply of NFTs listed on the marketplace
near view $MARKET_CONTRACT_ACTOR_ACCOUNT_ID get_supply_by_owner_id '{"account_id": "benji.testnet"}' # query for the total supply of NFTs listed by a specific owner on the marketplace
near view $MARKET_CONTRACT_ACTOR_ACCOUNT_ID get_supply_by_nft_contract_id '{"nft_contract_id": "fayyr-nft.testnet"}' # query for the total supply of NFTs that belong to a specific contract
near view $MARKET_CONTRACT_ACTOR_ACCOUNT_ID get_sale '{"nft_contract_token": "fayyr-nft.testnet.token-42"}' # query for important information for a specific listing or a sale object with its id
near view $MARKET_CONTRACT_ACTOR_ACCOUNT_ID get_sales_by_owner_id '{"account_id": "benji.testnet", "from_index": "5", "limit": 10}' # query for paginated information about the listings for a given owner
near view $MARKET_CONTRACT_ACTOR_ACCOUNT_ID get_sales_by_nft_contract_id '{"nft_contract_id": "fayyr-nft.testnet, "from_index": "5", "limit": 10}' # query for paginated information about the listings that originate from a given NFT contract