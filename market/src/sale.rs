



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









// --------------------------------
// --- payable function process ---
//      1 - ensure that the user has attached at least on yoctoNAER for the storage cost and security reasons like avoiding the DDOS attack on the contract by making sure that the caller has enough amo&unt to call this and is not an intruder
//      2 - then calculate the storage used in u64 bits or 8 bytes maximum (usize on 64 bits arch system) of mutating the state of the contract like mutating any collection inside the contract struct like the total size of a new entry added inside the collection or the total size of the removed entries 
//      3 - finally call something like refund_deposit() method to calculate the total costs for that bytes and refund to the caller any execess if there was an attached which was larger than the total storage cost or any removal entry process which will free up some storage which we must refund the caller based on the freed up storage bytes




// NOTE - if a function requires a deposit, we need a full access key of the user to sign that transaction which will redirect them to the NEAR wallet
// NOTE - gas fee is the computational fee paied as raward to validators by attaching them (in gas units) in scheduling function calls in which they mutate the state of the contract which face us cpu usage costs; and also the remaining deposit will get pay back as a refund to the caller by the near protocol
// NOTE - deposit or amount is the cost of the method and must be attached (in yocto$NEAR or near) for scheduling payable function calls based on storages they've used by mutating the state of the contract on chain like updating a collection field inside the contract struct and we have to get pay back the remaining deposit as a refund to the caller and that's what the refund_deposit() function does
// NOTE - if a contract method mutate the state like adding a data into a collection field inside the contract struct; the method must be a payable method (we need to tell the caller attach deposit to cover the cost) and we have to calculate the storage used for updating the contract state inside the function to tell the caller deposit based on the used storage in bytes (like the total size of the new entry inside a collection) then refund the caller with the extra tokens he/she attached
// NOTE - a payable method has &mut self as its first param and all calculated storage must of type u64 bits or 8 bytes maximum length (64 bits arch system usize)
// NOTE - caller in payable methods must deposit one yocto$NEAR for security purposes like always make sure that the user has some $NEAR in order to call this means only those one who have $NEARS can call this method to avoid DDOS attack on this method
// NOTE - a payable method can be used to pay the storage cost, the escrow price or the gas fee and the excess will be refunded by the contract method or the NEAR protocol
// NOTE - gas fee is the computational cost which must be paid if we’re doing cross contract call or moving between shards and actor cause this action will cost some cpu usage performance and must be attached separately in its related call from the cli 
// NOTE - amount or deposit is the cost of the payable function which can be either the cost of the storage usage for mutating contract or the cost of some donation or escrow ops
// NOTE - all payable methods needs to deposit some yocto$NEAR since they might be mutations on contract state and ensuring that the user is not DDOSing on the method thus the cost must be paid by the caller not by the contract owner and will refunded any excess that is unused
// NOTE - a view method can also force the user to attach yocto$NEAR to the call to prevent contract from DDOSing
// NOTE - if a method of the contract is going to mutate the state of the contract the first param of that method must be &mut self and it can be a none payable method like private method
// NOTE - in order to get the result of the cross contract call method we have to define a method inside the sender's or the caller's contract actor account by extending its contract struct interface by defining a trait which contains the definition of the callback method 
// NOTE - in order to call and schedule a promise or future object method from other contract actor account we have to define a trait and bound it to #[ext_contract()] proc macro which contains the method signature of the second contract actor account finally we can call in here and catch the the result of the scheduled promise of future object using the NEAR cross contract call syntax
// NOTE - callback methods inside the caller contract actor account must be defined private since no one except the caller contract can resolve the result of the executed promise scheduled in cross contract inside the receiver contract actor account, thus they must be defined as private methods   























// ----------------------------------------
//              SALE STRUCTURE
// ----------------------------------------
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate using the #[serde()] proc macro attribute itself
pub struct Sale{
    pub owner_id: AccountId, //-- the owner_id of this sale object or the NFT which is the seller account_id 
    pub approval_id: u64, //-- market contract actor approval_id to transfer the NFT on behalf of the owner 
    pub nft_contract_id: String, //-- the account_id that the NFT was minted on or it's current place inside a contract actor account which might be the account of the minter on first sell and current owner on later sales which is the seller
    pub token_id: TokenId, //-- the NFT id
    pub sale_consitions: SalePriceInYoctoNear, //-- the price of the listed NFT in yocto$NEAR 
}

















// ----------------------------------------
//     CROSS CONTRACT CALLS' INTERFACES
// ----------------------------------------
#[ext_contract(nft_contract)] //-- nft_contract name that we passed in #[ext_contract()] proc macro is the name of the contract (a hypothetical contract name of course) that we're extending its interface for cross contract call and creating transaction which is a promise (future object) ActionReceipt object and means we want to call the following methods inside that contract which contains a transaction which is a promise (future object) ActionReceipt object that must be executed later
trait NftContractReceiver{ //-- this trait which contains the cross conract call methods will extend the interface of the receiver_id's contract actor with a name inside the #[ext_contract()] proc macro which specifies the extended interface contract name on this contract 

    /////
    /////// ➔ following method must be called and executed inside the receiver_id's contract actor account (thus it must be existed and defined on receiver contract actor account) from this contract actor account therefore it'll take a param called account_id which is the one who should call this method on his/her contract actor account and must be the owner of his/her contract
    /////// ➔ receiver_id: purchaser (person to transfer the NFT to) | token_id: the id of the NFT to transfer | approval_id: market contract's approval_id in order to transfer the token on behalf of the owner | memo: memo (to include some context) | balance: the price that the token was purchased for, this will be used in conjunction with the royalty percentages for the token in order to determine how much money should go to which account | max_len_payout: the maximum amount of accounts the market can payout at once (this is limited by gas fee) 
    ///// 
    fn nft_transfer_payout(&mut self, receiver_id: AccountId, token_id: TokenId, approval_id: u64, memo: String, balance: U128, max_len_payout: u64); //-- this method will be used for cross contract call on the receiver_id's contract actor (which must be implemented on the receiver_id's contract actor) once the nft_transfer_call() method is called and will return true if the token should be returned back to the sender

}


#[ext_contract(market_contract)] //-- market_contract name that we passed in #[ext_contract()] proc macro is the name of the contract (a hypothetical contract name of course) that we're extending its interface to get the result of cross contract call and creating transaction which is a promise (future object) DataReceipt object and means we want to get the result of the cross contract call (which is a promise (future object) ActionReceipt object which has been executed) using the following methods inside this contract
trait MarketContractReceiver{ //-- this trait which contains the cross conract call methods will extend the interface of the current contract actor which is the market's contract actor account with a name (market_contract) inside the #[ext_contract()] proc macro which specifies the extended interface contract name

    ////
    /////// ➔ we'll use this method as a callback inside this contract to get the result of the cross contract call the nft_transfer_payout() method which has been scheduled inside the process_purchase() method to be executed on a receiver contract actor account which will be the NFT contract of the NFT owner which is the seller
    ////
    fn resolve_purchase(&mut self, buyer_id: AccountId, price: U128) -> Promise; //-- resolves the pending DataReceipt object of the created and scheduled promise on this contract of the cross contract call to the receiver contract (NFT owner which is the seller), this is the callback from calling the nft_transfer_payout() cross contract call promise method that we want to await on and solve it inside the process_purchase() method which will analyze what happened in the cross contract call when nft_transfer_payout was called as part of the process_purchase method

}




















// -----------------------------------------------------------------------------
//      SALE & CROSS CONTRACT CALL METHODS OF THE MarketContract STRUCTURE 
// -----------------------------------------------------------------------------
#[near_bindgen] //-- implementing the #[near_bindgen] proc macro attribute on `MarketContract` struct to compile all its methods to wasm so we can call them in near cli
impl MarketContract{ //-- following methods will be compiled to wasm using #[near_bindgen] proc macro attribute 



    ////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////// ➔ PAYABLE METHODS //////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////////////////////////////////////////////////////////

    // -------------------------
    //      SELLER METHOD
    // -------------------------
    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute
    pub fn remove_sale(&mut self, nft_contract_id: AccountId, token_id: TokenId){ //-- since we're mutating the state of the contract (and due to the fact that payable methods' first param must be &mut self) by removing an entry from all collections on chain thus we must define the first param as &mut self - this method will remove a sale object from the market and only the owner of the NFT which has been listed can do this means the caller of this method must be the owner of the NFT which is the seller 
        assert_one_yocto(); //-- ensuring that the user has attached exactly one yocto$NEAR to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto$NEAR can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost
        let sale = self.internal_remove_sale(nft_contract_id, token_id); //-- getting the sale object that we've just removed it from every where on chain
        let caller_account_id = env::predecessor_account_id(); //-- getting the caller of this method which must be the NFT owner which is the seller
        if caller_account_id != sale.owner_id{ //-- if this fails, the remove sale will revert
            let panic_message = format!("The Caller Of This Method Which Is [{}] Is Not Seller Or The Owner Of The Sale Object (The Listed NFT On Market)! Thus Can't Remove The `sale` Object.", caller_account_id);
            env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }
    }

    // -------------------------
    //      SELLER METHOD
    // -------------------------
    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute
    pub fn update_price(&mut self, nft_contract_id: AccountId, token_id: TokenId, price: U128){ //-- since we're mutating the state of the contract (and due to the fact that payable methods' first param must be &mut self) by updating an entry inside the self.sales collection thus we must define the first param as &mut self - this method will update the sale object price which is in yocto$NEAR inside the self.sales collection and only the owner of the NFT which has been listed can do this means the caller of this method must be the owner of the NFT which is the seller 
        assert_one_yocto(); //-- ensuring that the user has attached exactly one yocto$NEAR to the call to pay for the storage and security reasons (only those caller that have at least 1 yocto$NEAR can call this method; by doing this we're avoiding DDOS attack on the contract) on the contract by forcing the users to sign the transaction with his/her full access key which will redirect them to the NEAR wallet; we'll refund any excess amount from the storage later after calculating the required storage cost
        let contract_id: AccountId = nft_contract_id.into(); //-- converting the nft_contract_id into the AccountId which will be used to create the unique sale id - the current place of the NFT which can be the contract actor account_id of the minter on first sell or another owner on later sales which is the seller 
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id); //-- creating the unique id for a sale object from the nft_contract_id and the token_id
        let caller_account_id = env::predecessor_account_id(); //-- getting the caller of this method which must be the NFT owner which is the seller
        match self.sales.get(&contract_and_token_id){ //-- getting the sale object related to the created unique sale id from the self.sales collection 
            Some(mut sale) => {
                if sale.owner_id != caller_account_id{
                    let panic_message = format!("The Caller Of This Method Which Is [{}] Is Not Seller Or The Owner Of The Sale Object (The Listed NFT On Market)! Thus Can't Update The `sale` Object.", caller_account_id);
                    env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                }
                sale.sale_consitions = price;
                self.sales.insert(&contract_and_token_id, &sale); //-- inserting the updated sale object related to a specific owner which is the seller by passing contract_and_token_id and sale object in their borrowed form to have them in later scopes - insert() method will update the value on second call if there was any entry with that key exists cause hashmap based data structures use the hash of the key to validate the uniquness of their values and we must use enum based storage key if we want to add same key twice but with different values in two different collections to avoid data collision 

            },
            None => {
                env::panic_str("Found No Sale"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            },
        }
    }

    //////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////// ➔ BUYER METHODS //////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////

    // -------------------------
    //       BUYER METHOD
    // -------------------------
    #[payable] //-- means the following would be a payable method and the caller must pay for that and must get pay back the remaining deposit or any excess that is unused at the end by refunding the caller account by our contract (something like refund_deposit() method) or the NEAR protocol - we should bind the #[near_bindgen] proc macro attribute to the contract struct in order to use this proc macro attribute
    pub fn offer(&mut self, nft_contract_id: AccountId, token_id: TokenId){ //-- since payable method first param must be &mut self cause they might change the state of the contract on chain we'e defined the first param as &mut self
        let deposit = env::attached_deposit(); //-- getting the attached deposit to this call
        if deposit < 0{
            env::panic_str("The Attached Deposit To This Call Must Be Greater Than 0"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
        }
        let contract_id: AccountId = nft_contract_id.into(); //-- converting the nft_contract_id into the AccountId which will be used to create the unique sale id - the current place of the NFT which can be the contract actor account_id of the minter on first sell or another owner on later sales which is the seller 
        let contract_and_token_id = format!("{}{}{}", contract_id, DELIMETER, token_id); //-- creating the unique id for a sale object from the nft_contract_id and the token_id
        let caller_account_id = env::predecessor_account_id(); //-- getting the caller of this method which must not be the NFT owner since the NFT owner can't offer on his/her own NFT
        match self.sales.get(&contract_and_token_id){ //-- getting the sale object related to the created unique sale id from the self.sales collection 
            Some(mut sale) => {
                if sale.owner_id == caller_account_id{ //-- the owner of the NFT can't bid on his/her own NFT
                    env::panic_str("The NFT Owner Can't Bid On His/Her Own NFT"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                }
                let price = sale.sale_consitions.0; //-- getting the first element of the U128 tuple struct
                if deposit < price{
                    let panic_message = format!("The Attached Deposit To This Call Must Be Greater Or Equal To The Current Price Of The NFT Which Is {:?}", price);
                    env::panic_str(panic_message.as_str()); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
                }
                let buyer_id = caller_account_id;
                self.process_purchase(contract_id, token_id, U128(deposit), buyer_id); //-- it'll return a promise without getting its result using a callback which might be solved or failed; it depends on the result of cross contract call inside the self.process_purchase() method - processing the purchase which will remove the liste NFT or the sale object from the market, transfer the NFT to the buyer_id and get the payout from the NFT contract which has deployed on the owner of the NFT, finally distribute royalties
            },
            None => {
                env::panic_str("Found No Sale"); //-- &str allocates low cost storage than the String which will get usize (usize is 64 bits or 24 bytes on 64 bits arch) * 3 (pointer, len, capacity) bytes; cause it's just the size of the str itself which is the total length of its utf8 bytes array on either stack, heap or binary which is equals to its length of utf8 bytes and due to its unknown size at compile time we must borrow it by taking a pointer to its location
            },
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////// ➔ MARKET METHODS //////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////

    // -------------------------
    //       MARKET METHOD
    // -------------------------
    #[private] //-- means the following would be a private method and the caller or the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method to mutate the state of the contract on chain must be the signer (who initiated and signed the contract)
    pub fn process_purchase(&mut self, nft_contract_id: AccountId, token_id: TokenId, price: U128, buyer_id: AccountId) -> Promise{ //-- since the removal process will mutate the state of the contract on chain; we've defined the first param of self.internal_remove_sale() method as &mut self thus we must define the first param of the self.process_purchase() method as &mut self too otherwise we'll face the error of: cannot borrow `*self` as mutable, as it is behind a `&` reference, so the data it refers to cannot be borrowed as mutable since we're calling a mutable method inside of it which is the self.internal_remove_sale() method therefore the first param must be &mut self - this method initiate a cross contract call to the nft contract, this will transfer the token to the buyer and return a payout object used for the market to distribute funds to the appropriate accounts

        /*

            -----------------------------------------------------------------------------
            
            1 - a buyer invokes the process_purchase method to buy and purchases an NFT on the market
            2 - the process_purchase method calls internal_remove_sale method and schedule a cross contract call (nft_transfer_payout method) to the NFT contract which has been deployed on the owner account_id
            3 - after removing sale object from on chain collections a cross contract call which is a transaction which is a promise (future object) ActionReceipt object is scheduled 
                    an ActionReceipt is created to call the nft_transfer_payout method on the receiver contract
                    a callback resolve_purchase is registered on sender_id's contract actor by creating a pending ActionReceipt
            4 - on the next block either in a same shard or other shard, the nft_transfer_payout method is executed on the receiver_id's contract actor and a DataReceipt is created
            5 - on the next block either in a same shard or other shard, the pending ActionReceipt from above is ready and the resolve_purchase callback is executed
        

            for every cross contract calls we have to extend the interface of our contract struct by impl a trait for that to define the cross contract call promise methods 
                process_purchase()    ----- inside the market's contract actor
                nft_transfer_payout() ----- inside the receiver_id's contract actor - it must already be defined in there so we can schedule it in caller contract (market) to be executed on receiver_id's contract actor (NFT owner which is the seller) account  
                resolve_purchase()    ----- inside the market's contract actor to solve and fill the pending promise ActionReceipt object with the promise DataReceipt object coming from the receiver_id's contract actor (NFT owner which is the seller) account

        
            process_purchase()    on [market's contract actor]                      -> true if the token was transferred from the sender_id's contract actor - schedule the nft_transfer_payout() cross contract call promise method to be executed later on receiver_id's contract actor (NFT owner which is the seller) account
            resolve_purchase()    on [market's contract actor]                      -> NFT price u128 in yocto$NEAR based on the result of the nft_transfer_payout() cross contract call promise method - get the result of the scheduled promise inside this method by solving it using .then() method
            nft_transfer_payout() on [NFT owner which is the seller contract actor] -> true if the token should be returned back to the sender otherwise false - execute this promise on receiver_id's contract actor
        
            -----------------------------------------------------------------------------

        */

        let sale = self.internal_remove_sale(nft_contract_id.clone(), token_id.clone()); //-- removing the listed sale object contains the NFT info from the market - cloning the nft_contract_id and the token_id to have them in later scopes 

        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        ////////////// ➔ defaulting GAS weight to 1, attached 1 yocto$NEAR deposit, and static GAS equal to the GAS for nft_transfer_payout
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        

        nft_contract::ext(nft_contract_id) //-- the account_id that this method must be called and executed inside since the account_id param is the one who is responsible for executing this call which is the NFT owner which is the seller contract actor account
            .with_attached_deposit(1) //-- we must attach 1 yocto$NEAR in the following cross contract call since inside the nft_transfer_payout() method we've enforced the caller to attach 1 yocto$NEAR for security reasons like prevent the contract call from DDOSing 
            .with_static_gas(GAS_FOR_NFT_TRANSFER) //-- the total gas fee which will be deposited in yocto$NEAR from the caller wallet for this transaction cross contract call
            .nft_transfer_payout( //-- initiating a corss contract call by creating a transaction which is a promise (future object) ActionReceipt object which must be executed on receiver_id's contract actor account (NFT owner which is the seller) to transfer the NFT to the buyer contract actor account and fulfill the pending DataReceipt future object (which is an async message) with the cross contract call result inside the resolve_purchase() callback method using .then() since the fulfilled DataReceipt future object contains a payout object used for the market to distribute funds to the appropriate accounts - - calling the nft_transfer_payout() cross contract call promise method on the receiver side (NFT owner which is the seller) from the extended receiver_id's contract actor interface which is `nft_contract`
                buyer_id.clone(), 
                token_id, 
                sale.approval_id, //-- passing the approval_id from removed sale object from the on chain market collections   
                "payout from market".to_string(), 
                price, //-- the price that the token was purchased for, this will be used in conjunction with the royalty percentages for the token in order to determine how much money should go to which account
                10 //-- this is the maximum amount of accounts the market can payout at once (the number is limited by gas fee since we may run out of gas fee by transferring $NEARs with a single attached gas to nft_resolve_transfer() callback method)
            ).then( //-- wait for the scheduled transaction which is a promise (future object) ActionReceipt object on the receiver_id's contract actor (NFT owner which is the seller) to finish executing to resolve it using .then() method inside resolve_purchase() method
                //////////
                /////// ➔ by default ext() method will be attached to the contract struct annotated with #[near_bindgen] which avoids the requirement to re-define the interface with #[ext_contract] 
                ///////    and the method that will be attached to the struct is the same as ext_contract as ext(..) so we can call Self::ext(...) which remove the need to redefine interfaces twice
                /////// ➔ defaulting GAS weight to 1, no attached deposit, and static GAS equal to the GAS for resolve transfer
                ////////////
                Self::ext(env::current_account_id()) //-- the account_id that this method must be called and executed inside which is the current_account_id() and is the one who owns this contract which is the market itself - account_id param is the one who is responsible for executing this call which is the market itself
                    .with_attached_deposit(NO_DEPOSIT) //-- no deposit is required from the caller for calling the nft_resolve_transfer() callback method since this method doesn't require any deposit amount
                    .with_static_gas(GAS_FOR_RESOLVE_PURCHASE) //-- total gas required for calling the callback method which has taken from the attached deposited (contract budget) when the caller called the nft_transfer_call() method
                    .resolve_purchase( //-- calling resolve_purchase() method from the extended interface of the current contract actor (market contract) which is the `market_contract` contract; since this is a private method only the owner of the this contract can call it means the caller must be the signer or the one who initiated, owned and signed the contract or the account of the contract itself which is the market itself; since callback methods are private thus the caller of them must be the owner of the contract
                        ////
                        /////// ➔ the buyer_id and the price are passed in incase something goes wrong and we need to refund the buyer
                        ////
                        buyer_id,
                        price //-- passing the price to calculate the perpetual royalties
                    ) //-- resolve_purchase() method will return a U128 price which is in yocto$NEAR
            ) //-- returning the promise from this method 
        

    }

    // -------------------------
    //       MARKET METHOD
    // -------------------------
    #[private] //-- means the following would be a private method and the caller or the predecessor_account_id which is the previous contract actor account and the last (current) caller of this method to mutate the state of the contract on chain must be the signer (who initiated and signed the contract)
    pub fn resolve_purchase(&mut self, buyer_id: AccountId, price: U128) -> U128{ //-- this method is a private method which will be used as a callback to handle the result of the executed nft_transfer_payout() promise or future object which will take the payout object and check to see if it's authentic and there's no problems, if everything is fine, it will pay the accounts, otherwise it will refund the buyer for the price he/she has paid for 


        // reason we're using &mut self
        // why we're returning u128 in here????
        
        ////
        /////// ➔ actors will send utf8 encoded data through the mpsc channel, so we have to deserialize them when we resolve them outside of the fulfilled future object like deserializing the msg param which has been passed to the nft_on_approve() on the market contract actor account method inside the nft_approve() method on the NFT contract actor account
        /////// ➔ the following code flow is based on successful result of everything
        ////
        let payout_result = promise_result_as_success().and_then(|value|{ //-- promise_result_as_success() function uses env::promise_result() function under the hood - getting the result of the executed promise, the nft_transfer_payout() cross contract call; if it was successful we have the value in utf8 encoded form (since data between actors will be sent asyncly and serialized through the mpsc channel) which we have to deserialize it otherwise we'll get the None if the result of the promise wasn't successful
            serde_json::from_slice::<Payout>(&value) //-- deserializing the encoded payout object in form utf8 into the Payout struct
                .ok() //-- get the deserialized payout object only if the deserialization was ok
                .and_then(|payout_object|{ //-- and_then() returns an Option of either the parent method result or the result of the passed in closure which in our case we've passed in a closure with deserialized payout object as its arg
                    if payout_object.payout.len() > 10 || payout_object.payout.is_empty(){
                        
                    }
                })
        });
        






    }

    /////////////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////
    ////////////////////////////////// ➔ VIEW METHODS //////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////

    









}