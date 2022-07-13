





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

            https://www.near-sdk.io/contract-structure/collections ➔ Near Sdk Collection Performence
            https://stackoverflow.com/questions/72138820/near-marketplace-how-should-i-charge-the-transaction-fee-on-each-sales



*/







use serde_json::json;
use std::future;
use std::{fmt, collections::HashMap};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize}; //-- self referes to the borsh struct itself cause there is a struct called borsh inside the borsh.rs file
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet}; //-- LookupMap and UnorderedMap are non-iterable implementations of a map that stores their contents directly on the trie - LazyOption stores a value in the storage lazily! 
use near_sdk::json_types::{Base64VecU8, U128, U64}; //-- Base64VecU8 is used to serialize/deserialize Vec<u8> to base64 string
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ 
                env::STORAGE_PRICE_PER_BYTE, //-- loading the price of each byte in yocto$NEAR
                Gas, ext_contract, PromiseResult, env, near_bindgen, assert_one_yocto, //-- we're using the assert_one_yocto() function from the near_sdk cause it's using the env::panic_str() one the background 
                AccountId, Balance, CryptoHash, Promise, //-- Promise struct is needed to handle async cross contract calls or message passing between contract actors
                PanicOnDefault, PromiseOrValue, BorshStorageKey //-- PanicOnDefault macro must be used in case that the contract is required to be initialized with init methods which will be paniced on implemnted Default trait for the contract
            }; 






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











// NOTE - HashMap keeps all data in memory, to access it, the contract needs to deserialize the whole map and it deserializes (and serializes) the entire collection in one storage operation; accessing the entire collection is cheaper in gas than accessing all elements through N storage operations
// NOTE - try to validate the input, context, state and access using require! before taking any actions; the earlier you panic, the more gas you will save for the caller
// NOTE - borsh is used for internal STATE serialization and serde for external JSON serialization
// NOTE - if a function requires a deposit, we need a full access key of the user to sign that transaction which will redirect them to the NEAR wallet
// NOTE - gas fee is the computational fee paied as raward to validators by attaching them (in gas units) in scheduling function calls in which they mutate the state of the contract which face us cpu usage costs; and also the remaining deposit will get pay back as a refund to the caller by the near protocol
// NOTE - deposit or amount is the cost of the method and must be attached (in yocot$NEAR or near) for scheduling payable function calls based on storages they've used by mutating the state of the contract on chain like updating a collection field inside the contract struct and we have to get pay back the remaining deposit as a refund to the caller and that's what the refund_deposit() function does
// NOTE - if a contract method mutate the state like adding a data into a collection field inside the contract struct; the method must be a payable method (we need to tell the caller attach deposit to cover the cost) and we have to calculate the storage used for updating the contract state inside the function to tell the caller deposit based on the used storage in bytes (like the total size of the new entry inside a collection) then refund the caller with the extra tokens he/she attached
// NOTE - a payable method usaully has &mut self as its first param and all calculated storage must of type u64 bits or 8 bytes maximum length (64 bits arch system usize)
// NOTE - caller in payable methods must deposit one yocot$NEAR for security purposes like always make sure that the user has some $NEAR in order to call this means only those one who have $NEARS can call this method to avoid DDOS attack on this method
// NOTE - a payable method can be used to pay the storage cost, the escrow price or the gas fee
// NOTE - gas fee is the computational cost which must be paid if we’re doing cross contract call or moving between shards and actor cause this action will cost some cpu usage performance and must be attached separately in its related call from the cli 
// NOTE - amount or deposit is the cost of the payable function which can be either the cost of the storage usage for mutating contract or the cost of some donation or escrow ops
// NOTE - every cross contract calls for communicating between contract actor accounts in cross sharding pattern takes up cpu usage and network laoding costs which forces us to attach gas units in the contract method call in which the cross contract call method is calling to pass it through the calling of the cross contract call method
// NOTE - The NEAR whitepaper mentions that 30% of all gas fees go to smart contract accounts on which the fees are expensed
// NOTE - whenever a function is called an ActionReceipt object will be created by NEAR runtime from the transaction in which the state will be loaded and deserialized, so it's important to keep this amount of data loaded as minimal as possible
// NOTE - all payable methods needs to deposit some yocot$NEAR since they might be mutations on contract state and ensuring that the user is not DDOSing on the method thus the cost must be paid by the caller not by the contract owner and will refunded any excess that is unused
// NOTE - we can't impl Default trait for the contract if the PanicOnDefault trait is implemented for that contract
// NOTE - near hashmap and set based data structures or collections are LookupMap, LookupSet, UnorderedMap, UnorderedSet and TreeSet; each of them will be cached on chain to minimize the amount of gas used the SDK collections should be used in most cases instead of deserializing all entries each time the state and the app runtime is loaded like HashMap
// NOTE - current_account_id()     -> the id of the account that owns the current contract actor account
// NOTE - predecessor_account_id() -> the id of the account that was the previous contract actor account in the chain of cross-contract calls and if this is the first contract, it is equal to signer_account_id - the last (current) caller of a contract actor method which created and signed the transaction by calling that method
// NOTE - signer_account_id()      -> the id of the account that either signed the original transaction or issued the initial cross-contract call that led to this execution 
// NOTE - since mutating the contract state on the chain will cost money thus in order to list an NFT on the market we have to create a sell object which is an object contains the NFT info for listing it on the market, since by listing the NFT we're mutating the state of the `MarketContract` on chain thus we must force the seller to deposit the storage cost for listing his/her NFT on the market by calling the storage_deposit() method 












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
    pub sales: UnorderedMap<ContractAndTokenId, Sale>, //-- keeping the track of all the sales by mapping the ContractAndTokenId to a sale cause every sale has a unique identifier which is made up of the `contract actor account_id + DELIMETER + token_id` 
    pub by_owner_id: LookupMap<AccountId, UnorderedSet<ContractAndTokenId>>, //-- keeping the track of all the sale ids which is made up of the `contract actor account_id + DELIMETER + token_id` inside a set for every account_id
    pub by_nft_contract_id: LookupMap<AccountId, UnorderedSet<TokenId>>, //-- keeping the track of all the token_ids inside a set for a sale of a given account_id
    pub storage_deposits: LookupMap<AccountId, Balance>, //-- mapping between all the storages paid in yocto$NEAR of type u128 by a specific account_id
}


#[near_bindgen]
impl MarketContract{ //-- we'll add bytes to the contract by creating entries in the data structures - we've defined the init methods of the `MarketContract` struct in here cause the lib.rs is our main crate


    #[init] //-- means the following would be a contract initialization method which must be called by the contract owner and verifies that the contract state doesn't exist on chain since can only be called once and will be paniced on second call
    pub fn new(owner_id: AccountId) -> Self{ //-- initialization function can only be called once when we first deploy the contract to runtime shards - this initializes the `MarketContract` on chain with the passed in owner_id
        let accounts_message = format!("current account id is @{} | predecessor or the current caller account id is @{} | signer account id is @{}", env::current_account_id(), env::predecessor_account_id(), env::signer_account_id()); //-- format!() returns a String which takes 24 bytes storage, usize * 3 (pointer, len, capacity) bytes (usize is 64 bits or 24 bytes on 64 bits arch)
        // let accounts_message_bytes = accounts_message.as_bytes(); //-- as_bytes() returns &[u8] 
        env::log_str(&accounts_message); //-- passing the message in form of a borrowed type even though as_bytes() returns &[u8]
        Self{ //-- the return type is of type Self or the contract itself with initialized fields - this function will default all the collections to be empty
            owner_id,
            sales: UnorderedMap::new(Storagekey::Sales.try_to_vec().unwrap()),  //-- UnorderedMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called Sales defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the UnorderedMap sales keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"sales" which is the name of the collection field
            by_owner_id: LookupMap::new(Storagekey::ByOwnerId.try_to_vec().unwrap()),  //-- LookupMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called ByOwnerId defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the LookupMap by_owner_id keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"by_owner_id" which is the name of the collection field
            by_nft_contract_id: LookupMap::new(Storagekey::ByNFTContractId.try_to_vec().unwrap()),  //-- UnorderedMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called ByNFTContractId defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the LookupMap by_nft_contract_id keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"by_nft_contract_id" which is the name of the collection field
            storage_deposits: LookupMap::new(Storagekey::StorageDeposits.try_to_vec().unwrap()),  //-- UnorderedMap takes a unique vector of u8 bytes (to have unique encoding we've used an enum variant called StorageDeposits defined in utils::Storagekey) in it constructor argument as the prefix that must be append before the LookupMap storage_deposits keys to avoid data collision with other keys of other collections of the `MarketContract` fields since they might be same keys inside two different collection - the prefix can be also the utf8 encoded of a unique string like b"storage_deposits" which is the name of the collection field
        }
    }


    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute  
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>){ //-- since we're mutating the state of the contract by adding a new entry into the storage_deposit collection attached from the caller for selling an NFT sell object thus we must define the first param as &mut self with an optional account_id who wants to pay for storage cost of an allocated sale object on chain which can be either the seller or anyone who wants to pay for another contract actor account_id - this method will cover the cost of storing sale object on the contract on chain 
        let storage_deposit = env::attached_deposit(); //-- getting the attached deposit to the call by the caller in yocot$NEAR which is of type u128 - the required cost per sell object is 0.01 $NEAR or 10^19 in yocto$NEAR which will be deposited on chain inside the storage_deposit collection
        let storage_account_id = account_id
                                                .map(|a| a.into()) //-- mapping the account_id inside the Option to convert it into a valid account_id using .into() method which will return the T
                                                .unwrap_or_else(env::predecessor_account_id); //-- using the current caller account_id which might be the seller or anyone who wants to deposit the storage cost for a sell object related to a sepecific NFT
        if storage_deposit >= STORAGE_PER_SALE{ //-- making sure that the deposited amount
            let panic_message = format!("The Minimum Deposit Must be {} Which Is The Amount of Storing One Byte On The NEAR Chain", STORAGE_PER_SALE);
            env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }
        let mut account_id_balance = self.storage_deposits.get(&storage_account_id).unwrap_or(0); //-- getting the total deposited storage out of the Option using map of the passed in account_id and if the account_id wasn't inside the map we default to a balance of 0
        account_id_balance += storage_deposit; //-- updating the current balance of the passed in account_id with the deposited storage
        self.storage_deposits.insert(&storage_account_id, &account_id_balance); //-- inserting the updated balance related to the passed in account_id by passing storage_account_id and account_id_balance in their borrowed form to have them in later scopes - insert() method will update the value on second call if there was any entry with that key exists cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice but with different values in two different collections to avoid data collision
    }


    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute 
    pub fn storage_withdraw(&mut self){ //-- since we're mutating the state of the contract by removing an entry from the storage_deposit collection thus we must define the first param as &mut self - this method allows users (which can be sellers or anyone who has paid for the stroage cost of the sell object related to an NFT) to withdraw any excess storage that they're not using by the allocated sell object since the sell object might be sold out and no need to list it for the last seller anymore on the chain 
        assert_one_yocto(); //-- ensuring that the user has attached exactly one yocot$NEAR to the call to pay for the storage and security reasons (only those caller that have at least 1 yocot$NEAR can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess storage later
        let owner_id = env::predecessor_account_id(); //-- getting the account_id of the current caller which is the owner of the withdraw process
        let mut all_current_storage_deposited_amount = self.storage_deposits.remove(&owner_id).unwrap_or(0); //-- getting the total deposited amount for the current caller of this method to remove it from the map and if it wasn't any account_id matches with this caller we simply fill the amount with 0  
        let all_sale_ids_for_the_caller = self.by_owner_id.get(&owner_id); //-- getting the set of all the sale objects ids which is of type String for the current caller of this method
        let length_of_all_sale_ids = all_sale_ids_for_the_caller.map(|s| s.len()).unwrap_or_default(); //-- getting the total length of the sale object ids inside the set by mapping the wrapped UnorderedSet<String> into a none wrapped type to get its length and default will be set if there wasn't any sale object id inside the set  
        let total_storage_deposited_amount_for_all_sales = u128::from(length_of_all_sale_ids) * STORAGE_PER_SALE; //-- getting the total $NEARs which is being used up for all the current sale objects for the current caller of this method on this contract in yocto$NEAR which is of type u128 
        all_current_storage_deposited_amount -= total_storage_deposited_amount_for_all_sales; //-- the amount of excess to withdraw is the total storage that the current caller of this method has paid minus the storage that is being used up for all the sale objects for the current caller of this method
        
        if all_current_storage_deposited_amount > 0{
            
            
            
            // when an NFT is sold out we have to release the allocated storage by the sell object related to that NFT on the chain thus we have to payout the seller the amount of he/she deposited before, for the his/her sell object and he/she must withdraw the amount; the market contract actor account must have enough balance to pay the withdrawer
            // transferring the excess amount of storage deposit from the market contract actor account to the current caller of this method or the withdrawer
            // scheduling a transferring promise or future action receipt object to be executed later by the NEAR protocol which contains an async message which is the process of transferring NEARs to another contract actor account



        }
        if total_storage_deposited_amount_for_all_sales > 0{

        }



       

    }


    pub fn storage_minimum_balance(&self) -> U128{
        U128(STORAGE_PER_SALE) //-- returning the amount of the storage required per each sell in yocto$NEAR
    }


    pub fn storage_balance_of(&self, account_id: AccountId) -> U128{
        U128(self.storage_deposits.get(&account_id).unwrap_or(0)) //-- passing the account_id in its borrowed type - returning the total deposited storage for the passed in account_id in form u128 which is the type of yocto$NEAR
    }


    



}