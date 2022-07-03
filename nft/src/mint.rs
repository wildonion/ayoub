


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






use crate::{*, events::EventLogVariant}; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case



/*  
    -----------------------------------------------------------------------------
                            STEPS ON MINTING THE NFT

                    -> storage cost for 100 kb is 1 $NEAR <-    
    -----------------------------------------------------------------------------
    1) Calculate the initial storage before adding anything to the contract
    2) Create a Token object with the owner_id
    3) Link the token_id to the newly created token object by inserting them into the tokens_by_id field.
    4) Link the token_id to the passed in metadata by inserting them into the token_metadata_by_id field.
    5) Add the token_id to the list of tokens that the owner owns by calling the internal_add_token_to_owner function.
    6) Calculate the final and net storage to make sure that the user has attached enough NEAR to the call in order to cover those costs.


    NOTE - the total storage used by the following method will be calculated after calling the internal_add_token_to_owner() method by subtracting the initial_storage_usage at the beginning of the method from the used or released storage after the call
    NOTE - any execess amount will be paid back to the caller or the owner of the NFT once he/she transferred the NFT to someone else since transferring the NFT will free up the approved_account_ids hashmap and set it to empty hashmap {} thus we have to pay the released storage back the owner or the sender of the NFT who paid for approved account   
    NOTE - in the following method we add a new entry into `Contract` struct collections means we mutate the state of the contract by allocating extra storage on chain to insert a new NFT into all related collections thus we have to pay for it from caller's deposit and refund the caller if there was any execess storage cost 
    NOTE - taking all the available on chain storage in contract needs more $NEARs cause, the contract tracks the change in storage before and after the call
    NOTE - if the storage increases, the contract requires the caller of the contract to attach enough deposit to the function call to cover the storage cost.
    NOTE - if the storage decreases, the contract will issue a refund for the cost of the released storage. the unused tokens from the attached deposit are also refunded, so it's safe to attach more deposit than required.

*/




















#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `Contract` struct to compile all its methods to wasm so we can call them in near cli
impl NFTContract{ //-- following methods will be compiled to wasm using #[near_bindgen] proc macro attribute 

    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute 
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId, perpetual_royalties: Option<HashMap<AccountId, u32>>){ //-- we've defined the self to be mutable and borrowed cause we want to mutate the state of token_metadata_by_id and tokens_by_id fields but don't want to lose the lifetime of the created instance of the contract after calling this method 

        let initial_storage_usage = env::storage_usage(); //-- storage_usage() method calculate current total storage usage as u64 bits or 8 bytes maximum (usize on 64 bits arch system) of this smart contract that this account would be paying for - measuring the initial storage being uses on the contract 
        let mut royalty = HashMap::new(); //-- creating an empty royalty hashmap to keep track of the royalty percentage value for each owner_id that is passed in into the nft_mint() method, the perpetual_royalties param
        
        match perpetual_royalties{ // NOTE - perpetual_royalties hashmap contains accounts that will get perpetual royalties whenever the token is sold, of course it has the owner or the minter or creator of the collection or the NFT in addition to some charity or collaborator account_ids to get paid them and the minter will get paid on second sell
            Some(royalties) => {
                if royalties.len() >= 6{ //-- making sure that the length of the perpetual royalties is below 7 since we won't have enough gas fee to pay out that many people after selling the NFT and getting the payout object from the NFT contract which is deployed on the minter contract actor acctount 
                    env::panic_str("You Are Allowed To Add Only 6 Royalties Per Token Minting!"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes cause it's just the size of the str itself on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location   
                }
                for (owner_id, royalty_percentage_value) in royalties{ //-- NOTE - no need to call iter() method on royalties hashmap since we only want to insert the key and the value of perpetual_royalties hashmap into the royalty hashmap thus we don't the borrowed type of key and value
                    royalty.insert(owner_id, royalty_percentage_value); //-- filling the royalty hashmap with the incoming perpetual royalties from the call
                }
            },
            None => {
                env::log_str("No Royalty Hashmap was Passed"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes cause it's just the size of the str itself on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            }
        }
        
        let token = Token{
            owner_id: receiver_id, //-- the receiver_id is the one that this NFT will be belonged to him/her 
            approved_account_ids: Default::default(), //-- creating an empty hashmap or {} for all approved account ids 
            next_approval_id: 0, //-- next approval id must be started from 0 when we're minting the token
            royalty, //-- a mapping between owner_ids or some charity or collaborator account_ids and their royalty percentage value to calculate the payout later for each owner based on the NFT amount - the old owner which can be the main owner or the minter or creator on second sell will get paid at the end - total perpetual royalties 
        };

        // utils::panic_not_self(); //-- the minter or the caller of this method must be the owner of the contract means the bluerangene itself can mint a new NFT which is the marketplace itself but for now anyone can mint a new NFT and let it be as it is :)

        if self.tokens_by_id.insert(&token_id, &token).is_none() == false{ //-- if the token was already minted and the hashmap wasn't None for that key (token_id) we have to panic - inserting the token_id and the token struct into the tokens_by_id field to make sure that we didn't mint this token before; insert() method will update the value on second call if there was any entry with that key exists cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice on different times with different values
            env::panic_str("Token already minted!"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes cause it's just the size of the str itself on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }

        self.token_metadata_by_id.insert(&token_id, &metadata); //-- inserting the token_id and its metadata into the token_metadata_by_id field
        self.internal_add_token_to_owner(&token.owner_id, &token_id); //-- passing the borrowed of token owner_id and its id - adding current token to the owner; it'll insert a new token with its id and the owner_id into the tokens_per_owner field
    

        //////////////////////////////////////////////////////////////////////////////////////////
        ////////////////// CONSTRUCTING THE MINT LOG AS PER THE EVENTS STANDARD //////////////////
        //////////////////////////////////////////////////////////////////////////////////////////
        let nft_mint_log = EventLog{ //-- emitting the minting event
            standard: NFT_STANDARD_NAME.to_string(), //-- the current standard
            version: NFT_METADATA_SPEC.to_string(), //-- the version of the standard based on near announcement
            event: EventLogVariant::NftMint(vec![NftMintLog{ //-- the data related with the minting event stored in a vector 
                owner_id: token.owner_id, // the owner of all the tokens that were minted; since it might be a collection minting
                token_ids: vec![token_id], //-- list of all minted token ids; since it might be a collection minting
                memo: None, //-- the memo message which is None
            }]),
        }; // NOTE - since we've implemented the Display trait for the EventLog struct thus we can convert the nft_mint_log instance to string to log the nft minting event info at runtime
        env::log_str(&nft_mint_log.to_string()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        //////////////////////////////////////////////////////////////////////////////////////////


        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage; // -- calculating the required storage in u64 bits or 8 bytes which is total used unitl now - the initial storage
        refund_deposit(required_storage_in_bytes); //-- depositing some $NEARs based on used bytes in the contract and get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account; if the caller didn't attach enough it'll panic 
    
    }

}