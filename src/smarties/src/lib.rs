







// #![allow(unused)] //-- will let the unused vars be there
#![macro_use] //-- apply the macro_use attribute to the root cause it's an inner attribute and will be effect on all things inside this crate 
mod utils; //-- or crate::utils
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();








// NOTE - for calling private method current_account_id must be equal to predecessor_account_id (account of the contract)
// NOTE - Box<T> is one of the smart pointers in the Rust standard library, it provides a way to allocate enough memory on the heap to store a value of the corresponding type, and then it serves as a handle, a pointer to that memory
// NOTE - Box<T> owns the data it points to; when it is dropped, the corresponding piece of memory on the heap is deallocated and we can use dereference operator to reach their contents
// NOTE - every method call is a transaction in smart contract concepts
// NOTE - since can't compile socket in lib (wasm and bpf) mode contracts can't interact with their outside worlds  thus we can't have whether tokio or any web framework
// NOTE - bytecodes like .wasm and .so are compiled codes (on RAM instructions) from other langs must be loaded into a buffer to execute them on RAM using VMs
// NOTE - this contract (a family tree contract) is our campaign in which will catch a commission from incoming lamports and transfer the rest to the family tree owner account
// NOTE - funder will send a transaction also contains some instruction data to transfer lamports from his/her address to our campaign address (escrow)
// NOTE - our campaign contract contains some methods like TransferingWithCommission(), LockWallet() and MakeCampaignEmpty()
// NOTE - our campaign contract's methods will be called on a specific event or condition and that's what a smart contract does!









#[near_bindgen] //-- implementing the near_bindgen attribute on Counter struct to compile to wasm
#[derive(Default, BorshDeserialize, BorshSerialize)] //-- the struct needs to implement Default trait which NEAR will use to create the initial state of the contract upon its first usage - need for serde and codec ops - deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain   
pub struct Counter{
    val: i8,
    signer: Box<str>, // TODO - use Box methods on signer field
}


#[near_bindgen]
impl Counter{


    // TODO - add async function
    // ...  


    pub fn get_num(&self) -> i8{ //-- we can't mutate the state of self.val cause self is not mutable
        self.val //-- returns 8bits signed integer of the counter value
    }


    pub fn increment(&mut self){ //-- we've defined the self to be as mutable cause we want to mutate the state of the self.val 
        self.val = i8::wrapping_add(self.val, 1); //-- we've used wrapping_add() method to add 1 to self.val in order to prevent the add operation from overflowing above 127
        let log_message = format!("Increased number to {}", self.val);
        env::log(log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
        utils::simd_ops(); //-- calling simd ops
    }


    pub fn decrement(&mut self){ //-- we've defined the self to be as mutable cause we want to mutate the state of the self.val 
        self.val = i8::wrapping_sub(self.val, 1); //-- we've used wrapping_sub() method to subtract 1 from self.val in order to prevent the add operation from overflowing below -128
        let log_message = format!("Decreased number to {}", self.val);
        env::log(log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
        utils::simd_ops(); //-- calling simd ops
    }


    pub fn reset(&mut self){
        self.val = 0;
        env::log(b"Reset counter to zero"); //-- converting the log message string to utf8 bytes by putting `b` behind it
    }


    pub fn batch_minting(&mut self){ //-- minting batch of tokens
        // TODO - use mint!() macro inside the utils.rs
        // ...
        
    }

    pub fn batch_transfer(&mut self){
        // TODO - transfer multiple tokens at once 
        // ...
    }

    pub fn token_uri(&mut self, token_id: String){
        // TODO - this is intended to be the token metadata uri
        // ...
    }

}













#[cfg(test)]
mod tests {
    
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::json_types::ValidAccountId; //-- a struct to validate account ID during serde or codec ops 
    use near_sdk::serde::export::TryFrom; //-- since this trait has implemented for ValidAccountId struct we can import it to call the try_from() method on the struct - to use the trait methods on the struct we have to import the trait itself in the crate where the struct lives



    fn to_valid_account(account: &str) -> ValidAccountId{
        ValidAccountId::try_from(account.to_string()).expect("Invalid account") //-- returns a ValidAccountId
    }


    fn get_context(predecessor: ValidAccountId) -> VMContextBuilder{ //-- a mock context and VM to run the compiled contract wasm
        let mut builder = VMContextBuilder::new(); //-- modifying the default context by creating a mock context for unit tests
        builder.predecessor_account_id(predecessor); //-- use the incoming ValidAccountId as the new predecessor for the mock context VM  
        builder
    }


    #[test]
    fn increment(){
        let context = get_context(to_valid_account("wildonion.testnet")); //-- getting a mock context on wildonion.testnet account id
        testing_env!(context.build()); //-- setting up the mock context on the testing environment
        let mut contract = super::Counter{val: 0};
        contract.increment();
        println!("Value after increment: {}", contract.get_num());
        assert_eq!(1, contract.get_num());
    }
    
    #[test]
    fn decrement(){
        let context = get_context(to_valid_account("wildonion.testnet")); //-- getting a mock context on wildonion.testnet account id
        testing_env!(context.build()); //-- setting up the mock context on the testing environment
        let mut contract = super::Counter{val: 0};
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset(){
        let context = get_context(to_valid_account("wildonion.testnet")); //-- getting a mock context on wildonion.testnet account id
        testing_env!(context.build()); //-- setting up the mock context on the testing environment
        let mut contract = super::Counter{val: 0};
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num());
        assert_eq!(0, contract.get_num());
    }

}
