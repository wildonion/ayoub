




use crate::*; // load all defined crates, structs and functions from the root crate which is lib.rs in our case



#[near_bindgen]
impl Contract{ //-- following methods are none payable methods and will be compiled to wasm using #[near_bindgen] attribute
    
    
    
    
    // NOTE - following are some nft queries that must be perform on our current contract cause we want to like list all tokens for an owner and we have them in self.tokens_per_owner field
    // NOTE -  we've borrowed the self in the following methods cause we don't want to lose the lifetime of the created instance from the contract struct after calling each method 
    //         by borrowing the self we'll prevent the instance from moving and have it inside the upcoming scope even after calling these methods



    
    pub fn nft_total_supply(&self){ //-- query for the total supply of nfts on the contract
        
        /*

            ...

        */
    
    }

    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>){ //-- query for the tokens on the contract regardless of the owner using pagination - we put from_index and limit params inside Option in order to have a default value for them on None match
        
        /*

            ...

        */

    }

    pub fn nft_supply_for_owner(&self, account_id: AccountId){ //-- query for total supply of nfts for a given owner

        /*

            ...

        */

    }
    
    pub fn nft_tokens_per_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken>{ //-- query for all the token for an owner using pagination - we put from_index and limit params inside Option in order to have a default value for them on None match 
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id); //-- getting the set of all tokens for the account_id the value of self.tokens_per_owner is an unordered set which contains the token_id took from a persistent storage (created from the account_id) as the prefix key; means that token_id inside the unordered set belongs to a unique storage inside the memory which is owned by the hash of the account_id
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set{ //-- can't use match cause the return type must be equal in each match arm and we have either an empty vector or an UnorderedSet of Strings
            tokens_for_owner_set
        } else{
            return vec![]; //-- means we didn't find any token set for the current account_id and we must return an empty vector
        };
        
        let start = u128::from(from_index.unwrap_or(U128(0))); //-- start pagination from from_index var or start from 0 of type u128
        tokens.iter() //-- iterating through the set of all tokens which is belongs to a unique prefix key inside the memory which is the hash of the account_id  
              .skip(start as usize) //-- skip `start` elements until `start` elements are skipped; usize can be either 32 bits or 64 bits long - it'll return an iterator so we can map over it
              .take(limit.unwrap_or(50) as usize) //-- yield `limit` elements until `limit` elements are yeilded; usize can be either 32 bits or 64 bits long - it'll return an iterator so we can map over it
              .map(|token_id| self.nft_token(token_id.clone()).unwrap()) //-- return the token info json for this token_id using self.nft_token() method - we have to clone the token_id in each iteration when passing it to the self.nft_token() method cause we don't want to lose its ownership after passing
              .collect() //-- collect all the token infos related to the current account_id
    }

}