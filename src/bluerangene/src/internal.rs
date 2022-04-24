





use crate::*; // load all defined crates, structs and functions from the root crate which is lib.rs in our case



// NOTE - methods and function in here don't need to be compiled to wasm cause they're internal functions
// NOTE - can't use #[payable] and #[private] attributes on a none #[near_bindgen] attribute on the Contract implementation 



pub fn hash_account_id(account_id: &AccountId) -> CryptoHash{ //-- we'll pass the account_id as a borrowed type to this function - account_id in CryptoHash format is a 32 bytes or 256 bits which will be 64 chars in hex
    let mut hash = CryptoHash::default(); //-- getting the default hash which will be 32 bytes of utf8 bytes (8 bits long for each)
    hash.copy_from_slice(&env::sha256(account_id.as_bytes())); //-- extending the defined hash with the borrowed type of the bytes of the hash of the account_id by converting its String into utf8 bytes first
    hash
}


pub fn refund_deposit(storage_used: u64){ //-- refunding the initial deposit based on the amount of storage that was used up - all balances are of type u128
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used); //-- getting the required cost to store the information based on specified balance which is of type u128 from the used storage - storage_byte_cost() is the balance needed to store one byte on chain
    let attached_deposit = env::attached_deposit(); //-- getting the attached deposit - attached_deposit() method will get the balance that was attached to the call that will be immediately deposited before the contract execution starts; this is the minimum balance required to call the nft_mint() method 0.1 $NEAR is attached and the caller will get refunded any excess that is unused at the end 
    assert!(required_cost <= attached_deposit, "Need {} yocto$NEAR to mint", required_cost); //-- 1 yocto is 10^-24
    let refund = attached_deposit - required_cost; //-- refunding the owner account by subtracting the required_cost from his/her attached_deposit in yocto$NEAR
    if refund > 1{ //-- if the refund was greater than 1 yocto$NEAR, means we have to get pay back the remaining deposit as a refund to the predecessor account or the signer of this contract - refund is of type u128 or 16 bytes
        Promise::new(env::predecessor_account_id()).transfer(refund); //-- transfer the refund to the predecessor account or the signer which is the one who is minting this NFT - we've created a Promise object here with the predecessor account to transfer some $NEARs asyncly 
    }
}


impl Contract{ //-- we've defined the internal_add_token_to_owner() method of the Contract struct in this crate cause this crate is related to all internal calculation functions and methods - we don't need to add #[near_bindgen] attribute on this impl cause this is a none exporting method and won't compile to wasm to call it 

    pub fn internal_add_token_to_owner(&mut self, account_id: &AccountId, token_id: &TokenId){ //-- we've defined the self to be mutable and borrowed cause we want to add the account_id and minted token info to tokens_per_owner field and have the isntance with a valid lifetime after calling this method on it - add the minted token to the set of token an owner has first
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| { //-- getting the set of token_id(s) for the given account out of the LookupMap or create a new set for the given account inside the closure
            UnorderedSet::new( //-- if the account (minter) doesn't have any tokens related to the token_id, we create a new unordered set to save the minted token_id for the current account_id as his/her first NFT 
            Storagekey::TokenPerOwnerInner{ //-- choosing a new unique prefix or key from the enum for the storage of the current collection which is the TokenPerOwnerInner variant struct - UnorderedSet will create a new set based on the selected storage key which is an enum variant in our case and takes the size of the TokenPerOwnerInner struct
                        account_id_hash: hash_account_id(&account_id), //-- getting the hash of the current account_id
                } //-- our current storage (also current variant) is the TokenPerOwnerInner struct
                .try_to_vec() //-- converting the selected storage key into a vector of u8
                .unwrap(),
            )
        }); //-- the type of the tokens_set must be UnorderedSet<String>
        tokens_set.insert(token_id); //-- inserting the token_id into the created set for the current account_id
        self.tokens_per_owner.insert(account_id, &tokens_set); //-- inserting the created set for the given account_id
    }

}
