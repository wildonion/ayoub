





/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  



*/







use serde_json::json;
use std::{fmt, collections::HashMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itset cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128, U64}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{Gas, ext_contract, PromiseResult, env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue, BorshStorageKey}; //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors - PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract
use crate::utils::*;
use crate::constants::*;
use crate::external::*;
use crate::nft_callbacks::*;
use crate::sale::*;
use crate::sale_views::*;
use crate::internal::*;






pub mod constants;
pub mod utils; //-- or crate::utils
pub mod internal;
pub mod external;
pub mod sale_views;
pub mod sale;
pub mod nft_callbacks;







// NOTE - The NEAR whitepaper mentions that 30% of all gas fees go to smart contract accounts on which the fees are expensed
// NOTE - whenever a function is called an ActionReceipt object will be created by NEAR runtime from the transaction in which the state will be loaded and deserialized, so it's important to keep this amount of data loaded as minimal as possible
// NOTE - all payable methods needs to deposit some yocot$NEAR since they might be mutations on contract state and ensuring that the user is not DDOSing on the method thus the cost must be paid by the caller not by the contract owner and will refunded any excess that is unused
// NOTE - we can't impl Default trait for the contract if the PanicOnDefault trait is implemented for that contract
// NOTE - near hashmap and set based data structures or collections are LookupMap, LookupSet, UnorderedMap, UnorderedSet and TreeSet; each of them will be cached on chain instead of deserializing all entries each time the state and the app runtime is loaded like HashMap  
// NOTE - current_account_id()     -> the id of the account that owns the current contract actor account
// NOTE - predecessor_account_id() -> the id of the account that was the previous contract actor account in the chain of cross-contract calls and if this is the first contract, it is equal to signer_account_id - the last (current) caller of a contract actor method which created and signed the transaction by calling that method
// NOTE - signer_account_id()      -> the id of the account that either signed the original transaction or issued the initial cross-contract call that led to this execution 












#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate
pub struct Payout{ //-- payout type for the royalty standards which specifies which account_id must get paid how much per each sell of a specific NFT
    pub payout: HashMap<AccountId, U128>, // NOTE - HashMap has loaded inside the lib.rs before and we imported using use crete::* syntax 
}








/*
 
  -----------------------------
 |          Contract 
  -----------------------------
 | FIELDS:
 |      owner_id --------------> this is the owner of the market contract
 |      sales
 |      by_owner_id
 |      by_nft_contract_id
 |      storage_deposits
 |

*/

#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] //-- borsh is need for serde and codec ops; deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain 
pub struct MarketContract{
    pub owner_id: AccountId, //-- keeping the track of the owner of the contract which is the one who has called the initialization method and sign the transaction
    pub sales: UnorderedMap<ContractAndTokenId, Sale>, //-- keeping the track of the sales by mapping the ContractAndTokenId to a sale cause it's a unique identifier for every sale which is made up of the `contract actor account_id + DELIMETER + token_id` 
    

}