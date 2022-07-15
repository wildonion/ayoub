



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





use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case


















#[near_bindgen]
impl MarketContract{ //-- we'll add bytes to the contract by creating entries in the data structures - we've defined the init methods of the `MarketContract` struct in here cause the lib.rs is our main crate
    
    
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
        assert_one_yocto(); //-- ensuring that the user has attached exactly one yocot$NEAR to the call to pay for the storage and security reasons (only those caller that have at least 1 yocot$NEAR can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost
        let owner_id = env::predecessor_account_id(); //-- getting the account_id of the current caller which is the owner of the withdraw process
        let all_current_storage_deposited_amount = self.storage_deposits.remove(&owner_id).unwrap_or(0); //-- getting the total deposited amounts for the current caller of this method to remove it from the map and if it wasn't any account_id matches with this caller we simply fill the amount with 0  
        let all_sale_ids_for_the_caller = self.by_owner_id.get(&owner_id); //-- getting the set of all the sale objects ids which is of type String for the current caller of this method
        let length_of_all_sale_ids = all_sale_ids_for_the_caller.map(|s| s.len()).unwrap_or_default(); //-- getting the total length of the sale object ids inside the set by mapping the wrapped UnorderedSet<String> into a none wrapped type to get its length and default will be set if there wasn't any sale object id inside the set  
        let total_storage_deposited_amount_for_all_sales = u128::from(length_of_all_sale_ids) * STORAGE_PER_SALE; //-- getting the total $NEARs which is being used up for all the current sale objects for the current caller of this method on this contract in yocto$NEAR which is of type u128 
        
        
        ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        ////////////// ➔ total amounts that has been deposited till now for listing NFTs by calling stroage_deposit() method - total amounts that is required to be deposited for all the sale objects inside the set related to a specific contract actor account
        ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        let updated_current_storage_deposited_amount = all_current_storage_deposited_amount - total_storage_deposited_amount_for_all_sales; //-- the amount of excess to withdraw is the total storage that the current caller of this method has paid till now minus the storage that is being used up for all the sale objects for the current caller of this method


        if updated_current_storage_deposited_amount > 0{ //-- once an NFT has been sold out we have to release the allocated storage by the sell object related to that NFT on the chain thus we have to payout the seller the amount of he/she deposited before, for the his/her sell object and he/she must withdraw the amount; the market contract actor account must have enough balance to pay the withdrawer
            Promise::new(owner_id.clone()).transfer(updated_current_storage_deposited_amount); //-- transferring the excess amount of storage deposit from the market contract actor account budget to the current caller of this method or the withdrawer - scheduling a transferring promise or future action receipt object to be executed later by the NEAR protocol which contains an async message which is the process of transferring NEARs to another contract actor account
        }


        if total_storage_deposited_amount_for_all_sales > 0{
            
            /*

                ----------------------------------------------------------------------------------------------------------

                since the amount of deposited storage from a seller might be higher (calling self.storage_deposit() 
                for each listing or for example attach 10 $NEARs in just one call) than the storage amount of total 
                sale objects related to that seller (since the seller might had deposited too many $NEARs for the 
                storage but only a few of his/her NFTs has sold out thus his/her total deposited amounts inside the 
                self.storage_deposits is greater than the total amount of his/her remaining sale objects calculated 
                using the length of all sale objects inside the self.by_owner_id collection) thus we have to send 
                back the total amounts of all remaining sale objects inside the self.deposit_storages if the amount 
                of them is only greater than 0 since there are other listed NFTs in there that are not sold out yet!


                for example: a seller deposits 10 $NEARs for his sell which covers 1000 listings on the market
                            since the creator must approve the market contract actor account by calling the nft_approve() method
                            on the NFT contract for a specific NFT thus we might have only a few NFTs which are approved to be listed
                            and transferred on behalf of the owner or the creator on the market; but the seller has deposited 1000 listings 
                            which will cover the future listings also; therefore the total deposited amounts for a sepecific account_id
                            inside the self.storage_deposits collection might be greater than the total amounts of the listing NFTs 
                            or all sale objects listed inside the self.by_owner_id collection and due to this fact we must insert
                            the total amounts of all sale objects related to the seller back to the self.storage_deposits collection
                            if it was greater than 0; by doing this we're updating the amount of deposited_storage for the seller 
                            with the amount of all remaining sale objects to make sure that the seller has covered the cost of 
                            his/her other listings on the market inside the self.deposit_storages collection on chain and if the user 
                            had 500 sale objects listed remaining on the market, we insert that value here so if those sales 
                            get taken down, the user can then go and withdraw 500 sales worth of storage.

                
                NOTE - listing an NFT is done first by calling the nft_approve() method of the NFT contract actor account
                       which has been deployed on the creator account and second by scheduling a future object inside the
                       mentioned method to be executed on the market contract actor account inside the nft_on_approve() method
                       through a cross contract call.
                
                NOTE - by executing the nft_on_approve() method on the market contract actor account an NFT will be listed 
                       to be sold out on the market on chain on behalf of the owner or the creator. 

                ----------------------------------------------------------------------------------------------------------

            */
            
            self.storage_deposits.insert(&owner_id, &total_storage_deposited_amount_for_all_sales); //-- inserting the total storage deposited amount for all sales back into the storage_deposits if its greater than 0 - passing the owner_id and the amount of all sale objects by reference and no need to clone the owner_id cause we won't use it in later scopes  
        }


    }


    pub fn storage_minimum_balance(&self) -> U128{ //-- view method to see the minimum balance required for a storing one sale object
        U128(STORAGE_PER_SALE) //-- returning the amount of the storage required per each sell in yocto$NEAR
    }


    pub fn storage_balance_of(&self, account_id: AccountId) -> U128{ //-- view method to see the total balance deposited for the storage of a specific owner_id
        U128(self.storage_deposits.get(&account_id).unwrap_or(0)) //-- passing the account_id in its borrowed type - returning the total deposited storage for the passed in account_id in form u128 which is the type of yocto$NEAR
    }


}
















