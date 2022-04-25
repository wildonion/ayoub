




use serde_json::json;
use std::collections::HashMap;
use std::{sync::mpsc, thread, time::Instant}; // NOTE - mpsc means multiple thread can access the Arc<Mutex<T>> but only one of them can mutate the T
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itset cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{Gas, ext_contract, log, PromiseResult, env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue}; //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors - PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract
use crate::utils::*;
use crate::approval::*;
use crate::enumeration::*;
use crate::metadata::*;
use crate::mint::*;
use crate::nft_core::*;
use crate::royalty::*;
use crate::internal::*;
use crate::constants::*;




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











#[near_bindgen] //-- implementing the near_bindgen attribute on Counter struct to compile to wasm
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)] //-- borsh is need for serde and codec ops; deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain   
pub struct Contract{ //-- can't implement Default trait for this contract cause Default is not implemented for LazyOption, LookupMap and UnorderedMap structs - our contract keeps track of some mapping between owner_id, token_id and the token_metadata inside some collections
    pub owner_id: AccountId, //-- contract owner
    pub metadata: LazyOption<NFTContractMetadata>, //-- keeps track of the metadata for the contract
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, //-- keeps track of all the token_ids for a given account using a none iterable map in a LookupMap collection
    pub tokens_by_id: LookupMap<TokenId, Token>, //-- keepts track of the token struct for a given token_id using a none iterable map in a LookupMap collection
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, //-- keeps track of the token metadata for a given token_id using a none iterable map in an UnorderedMap collection
}



#[near_bindgen]
impl Contract{ //-- we'll add bytes to the contract by creating entries in the data structures - we've defined the init methods of the Contract struct in here cause the lib.rs is our main crate

    #[init] //-- means the following would be a contract initialization method and verifies that the contract state doesn't exist which can only be called once and will be paniced on second call
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with metadata that was passed in and the owner_id
        Self{ //-- the return type is of type Self or the contract itself with initialized fields - this function will default all the collections to be empty
            owner_id,
            metadata: LazyOption::new(Storagekey::NFTContractMetadata.try_to_vec().unwrap(), Some(&metadata)), //-- LazyOption takes a vector of u8 bytes from the current storage taken by NFTContractMetadata collection or variant
            tokens_per_owner: LookupMap::new(utils::Storagekey::TokensPerOwner.try_to_vec().unwrap()), //-- LookupMap takes a vector of u8 bytes from the current storage taken by TokensPerOwner collection or variant
            tokens_by_id: LookupMap::new(Storagekey::TokensById.try_to_vec().unwrap()), //-- LookupMap takes a vector of u8 bytes from the current storage taken by TokensById collection or variant
            token_metadata_by_id: UnorderedMap::new(Storagekey::TokenMetadataById.try_to_vec().unwrap()), //-- UnorderedMap takes a vector of u8 bytes from the current storage taken by TokenMetadataById collection or variant
        }
    }

    #[init] //-- means the following would be a contract initialization method and verifies that the contract state doesn't exist which can only be called once and will be paniced on second call
    pub fn new_default_meta(owner_id: AccountId) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with default metadata so the user don't have to manually type metadata
        Self::new( //-- calling new() method with some default metadata params and the owner_id passed in
            owner_id,
   NFTContractMetadata{
                spec: "nft-1.0.0".to_string(),
                name: "Family Tree Contract".to_string(),
                symbol: "FTC".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[private] //-- means the following would be a private method and the caller must be the signer to mutate the state of the contract on chain
    pub fn only_me(&mut self){
        if env::current_account_id() != env::predecessor_account_id(){ //-- the current caller is not me! or the signer of this contract
            env::panic("caller is not the signer".as_bytes()) //-- panic on runtime - all log messages in contract must be encoded or serialized into utf8 bytes to have low cost storage
        }
    }

}

