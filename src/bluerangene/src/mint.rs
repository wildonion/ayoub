


use crate::*; // load all defined crates, structs and functions from the root crate which is lib.rs in our case




// NOTE - here we'll define minting method of the Contract struct


/*  
    -----------------------------------------------------------------------------
    1) Calculate the initial storage before adding anything to the contract
    2) Create a Token object with the owner_id
    3) Link the token_id to the newly created token object by inserting them into the tokens_by_id field.
    4) Link the token_id to the passed in metadata by inserting them into the token_metadata_by_id field.
    5) Add the token_id to the list of tokens that the owner owns by calling the internal_add_token_to_owner function.
    6) Calculate the final and net storage to make sure that the user has attached enough NEAR to the call in order to cover those costs.
*/


#[near_bindgen]
impl Contract{

    #[payable] //-- means the following would be a payable method and the caller must pay for that 
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId){ //-- we've defined the self to be mutable and borrowed cause we want to mutate the state of token_metadata_by_id and tokens_by_id fields but don't want to lose the lifetime of the created instance of the contract after calling this method 
        
        let initial_storage_usage = env::storage_usage(); //-- storage_usage() method calculate current total storage usage of this smart contract that this account would be paying for - measuring the initial storage being uses on the contract as u64 bits or 8 bytes 
        let token = Token{
            owner_id: receiver_id, //-- the receiver_id is the one who is minting this token and is the owner of the current token
        };
    
        assert!(self.tokens_by_id.insert(&token_id, &token).is_none(), "Token already minted!"); //-- inserting the token_id and the token struct into the tokens_by_id field to make sure that we didn't mint this token before
        self.token_metadata_by_id.insert(&token_id, &metadata); //-- inserting the token_id and its metadata into the token_metadata_by_id field
        self.internal_add_token_to_owner(&token.owner_id, &token_id); //-- passing the borrowed of token owner_id and its id - adding current token to the owner; it'll insert a new token with its id and the owner_id into the tokens_per_owner field
    
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage; // -- calculating the required storage which is total used unitl now - the initial storage
        refund_deposit(required_storage_in_bytes); //-- the total amounts of the $NEAR based on used bytes in the contract
    
    }

}