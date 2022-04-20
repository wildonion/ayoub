






use std::collections::HashMap;
use std::mem::size_of;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itset cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue}; //-- Promise struct is needed to handle async returns in smart contract worlds!
use crate::utils::*;
use crate::approval::*;
use crate::enumeration::*;
use crate::metadata::*;
use crate::mint::*;
use crate::nft_core::*;
use crate::royalty::*;
use crate::internal::*;



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
#[derive(BorshDeserialize, BorshSerialize)] //-- the struct needs to implement Default trait which NEAR will use to create the initial state of the contract upon its first usage - need for serde and codec ops - deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain   
pub struct Contract{ //-- can't implement Default trait for this contract cause Default is not implemented for LazyOption, LookupMap and UnorderedMap structs
    pub owner_id: AccountId, //-- contract owner
    pub metadata: LazyOption<NFTContractMetadata>, //-- keeps track of the metadata for the contract
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, //-- keeps track of all the token IDs for a given account using a none iterable map
    pub tokens_by_id: LookupMap<TokenId, Token>, //-- keepts track of the token struct for a given token ID using a none iterable map
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, //-- keeps track of the token metadata for a given token ID using a none iterable map
}



#[near_bindgen]
impl Contract{ //-- we'll add bytes to the contract by creating entries in the data structures - we've defined the init methods of the Contract struct in here cause the lib.rs is our main crate

    #[init] //-- means the following would be a contract initialization method
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the contract with metadata that was passed in and the owner_id
        Self{ //-- the return type is of type Self or the contract itself with initialized fields - this function will default all the collections to be empty
            owner_id,
            metadata: LazyOption::new(Storagekey::NFTContractMetadata.try_to_vec().unwrap(), Some(&metadata)),
            tokens_per_owner: LookupMap::new(utils::Storagekey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(Storagekey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(Storagekey::TokenMetadataById.try_to_vec().unwrap()),
        }
    }

    #[init] //-- means the following would be a contract initialization method
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

}

