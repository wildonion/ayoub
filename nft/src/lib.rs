





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
use near_sdk::json_types::{Base64VecU8, U128}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{Gas, ext_contract, PromiseResult, env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue}; //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors - PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract
use crate::utils::*;
use crate::approval::*;
use crate::enumeration::*;
use crate::mint::*;
use crate::royalty::*;
use crate::metadata::*;
use crate::nft_core::*;
use crate::internal::*;
use crate::constants::*;
use crate::events::*;






pub mod constants;
pub mod utils; //-- or crate::utils
pub mod approval;
pub mod enumeration;
pub mod events;
pub mod metadata;
pub mod mint;
pub mod nft_core;
pub mod royalty;
pub mod internal;











// NOTE - this contract has an `owner_id` field which is the who is the signer and the owner of deploying process of this contract, also is the owner of all the NFTs that will be minted on this contract actor account to sell them on the marketplace
// NOTE - since this contract will be deployed on every contract actor account who wants to mint his/her all NFTs on his `account_id` to sell them on the marketplace thus the marketplace needs to be an approved `account_id` for the owner to transfer or list his/her all NFTs on behalf of him/her in there 
// NOTE - the marketplace can make a cross contract call to all implemented methods in this contract (which is deployed on minter or creator contract actor account_id) like approval and transfer methods to sell the NFT on behalf of the owner
// NOTE - our `NFTContract` has all the minted nfts info inside of it with a mapping between their metadata and the owner
// NOTE - `NFTContract` struct contains some data structures to store on chain infos about tokens and their owners at runtime
// NOTE - whenever a function is called an ActionReceipt object will be created by NEAR runtime from the transaction in which the state will be loaded and deserialized, so it's important to keep this amount of data loaded as minimal as possible
// NOTE - all payable methods needs to deposit some yocot$NEAR since they might be mutations on contract state and ensuring that the user is not DDOSing on the method thus the cost must be paid by the caller not by the contract owner and will refunded any excess that is unused
// NOTE - we can't impl Default trait for the contract if the PanicOnDefault trait is implemented for that contract
// NOTE - near hashmap and set based data structures or collections are LookupMap, LookupSet, UnorderedMap, UnorderedSet and TreeSet; each of them will be cached on chain instead of deserializing all entries each time the state and the app runtime is loaded like HashMap  
// NOTE - current_account_id()     -> the id of the account that owns the current contract actor account
// NOTE - predecessor_account_id() -> the id of the account that was the previous contract actor account in the chain of cross-contract calls and if this is the first contract, it is equal to signer_account_id - the last (current) caller of a contract actor method which created and signed the transaction by calling that method
// NOTE - signer_account_id()      -> the id of the account that either signed the original transaction or issued the initial cross-contract call that led to this execution 











/*
 
  -----------------------------
 |          Contract 
  -----------------------------
 | FIELDS:
 |      owner_id --------------> this is the minter account_id which this contract is deployed on and is the one who can mint all NFTs in here
 |      metadata
 |      tokens_per_owner
 |      tokens_by_id
 |      token_metadata_by_id
 |

*/

#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] //-- borsh is need for serde and codec ops; deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain   
pub struct NFTContract{ //-- can't implement Default trait for this contract cause Default is not implemented for LazyOption, LookupMap and UnorderedMap structs - our contract keeps track of some mapping between owner_id, token_id and the token_metadata inside some collections
    pub owner_id: AccountId, //-- contract owner who called the initialization method which can be anyone; this is the owner_id of the one which this contract must get deployed on to mint all his/her all NFTs on  
    pub metadata: LazyOption<NFTContractMetadata>, //-- keeps track of the metadata for the contract
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, //-- keeps track of all the token_ids for a given account using a none iterable map in a LookupMap collection - this field will be used to retrieve all nft_id(s) for a specific owner on this contract
    pub tokens_by_id: LookupMap<TokenId, Token>, //-- keepts track of the token struct (owner_id) for a given token_id using a none iterable map in a LookupMap collection - this field is a mapping between token_id(s) and the token object which contains the owner_id thus will be used to retrieve all owner_id(s) for a specific token_id on this contract
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, //-- keeps track of the token metadata for a given token_id using a none iterable map in an UnorderedMap collection - this field will be used to retrieve a token metadata for a specific token_id on this contract
}


#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `NFTContract` struct to compile all its methods to wasm so we can call them in near cli
impl NFTContract{ //-- we'll add bytes to the contract by creating entries in the data structures - we've defined the init methods of the `NFTContract` struct in here cause the lib.rs is our main crate

    #[init] //-- means the following would be a contract initialization method which must be called by the contract owner and verifies that the contract state doesn't exist on chain since can only be called once and will be paniced on second call
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with metadata that was passed in and the owner_id
        let accounts_message = format!("current account id is @{} | predecessor or the current caller account id is @{} | signer account id is @{}", env::current_account_id(), env::predecessor_account_id(), env::signer_account_id()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        // let accounts_message_bytes = accounts_message.as_bytes(); //-- as_bytes() returns &[u8] 
        env::log_str(&accounts_message); //-- passing the message in form of a borrowed type even though as_bytes() returns &[u8]
        Self{ //-- the return type is of type Self or the contract itself with initialized fields - this function will default all the collections to be empty
            owner_id,
            metadata: LazyOption::new(Storagekey::NFTContractMetadata.try_to_vec().unwrap(), Some(&metadata)), //-- LazyOption takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by NFTContractMetadata collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
            tokens_per_owner: LookupMap::new(utils::Storagekey::TokensPerOwner.try_to_vec().unwrap()), //-- LookupMap takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by TokensPerOwner collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
            tokens_by_id: LookupMap::new(Storagekey::TokensById.try_to_vec().unwrap()), //-- LookupMap takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by TokensById collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
            token_metadata_by_id: UnorderedMap::new(Storagekey::TokenMetadataById.try_to_vec().unwrap()), //-- UnorderedMap takes a vector of u8 bytes in it constructor argument as the prefix from the current storage taken by TokenMetadataById collection or the 64 bits (8 bytes) address of the enum tag which is pointing to the current variant
        }
    }

    #[init] //-- means the following would be a contract initialization method which must be called by the contract owner and verifies that the contract state doesn't exist on chain since can only be called once and will be paniced on second call
    pub fn new_default_meta(owner_id: AccountId) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with default metadata so the user don't have to manually type metadata
        Self::new( //-- calling new() method with some default metadata params and the owner_id passed in
            owner_id,
   NFTContractMetadata{
                spec: "nft-1.0.0".to_string(),
                name: "BlueRangene Family Tree NFT Contract".to_string(),
                symbol: "FTC".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[private] //-- means the following would be a private method and the caller or the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method to mutate the state of the contract on chain must be the signer (who initiated and signed the contract)
    pub fn only_owner(&mut self){
        utils::panic_not_self(); //-- panic on env::current_account_id() != env::predecessor_account_id() condition
    }


}