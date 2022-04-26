





use crate::*; // load all defined crates, structs and functions from the root crate which is lib.rs in our case




// NOTE - here we implement nft core queries trait like transfering and viewing the nft info for any contract struct
// NOTE - the reason behind the trait impl is that we can bound it to any given contract struct to use the nft methods on that contract struct  




pub trait NoneFungibleTokenCore{ //-- defining a trait for nft core queries, we'll implement this for any contract that wants to interact with nft core queries - this is not object safe trait cause we have generic params in its methods
    
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>); //-- transfering an nft from the current owner to a receiver_id - memo is a note!
    fn nft_transfer_call(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>, msg: String); //-- transfering an nft to a receiver_id - it'll call a function on the receiver_id's contract and return true if the token was transferred from the sender's account  
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>; //-- getting the information about an nft using its id

}




#[near_bindgen] //-- implementing the #[near_bindgen] attribute on the trait implementation for the Contract struct in order to have a compiled trait for this struct 
impl NoneFungibleTokenCore for Contract{ //-- implementing the NoneFungibleTokenMetadata trait for our main Contract struct (or any contract); bounding the mentioned trait to the Contract struct to query nft metadata infos

    #[payable] //-- means the following would be a payable method and the caller must pay for that 
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>){ //-- we've defined the self to be mutable and borrowed cause we want to mutate some fields and have the isntance with a valid lifetime after calling this method on it

        /*

            ...

        */

    }

    #[payable] //-- means the following would be a payable method and the caller must pay for that
    fn nft_transfer_call(&mut self, receiver_id: AccountId, token_id: TokenId, memo: Option<String>, msg: String){ //-- we've defined the self to be mutable and borrowed cause we want to mutate some fields and have the isntance with a valid lifetime after calling this method on it

        /*

            ...

        */

    }

    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>{
        if let Some(token) = self.tokens_by_id.get(&token_id){ //-- if there is some token object (contains owner_id info) with this id in the tokens_by_id collection - we can use match arm tho!
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap(); //-- getting the metadata for this token using its id - we're passing the borrowed type of the token_id in order to have it inside other scope and its lifetime be valid
            Some(JsonToken{
                token_id,
                owner_id: token.owner_id,
                metadata,
            })
        } else{ //-- if there wasn't a token with this id in tokens_by_id collection we return None
            None
        }
    }

}