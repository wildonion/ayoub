




// #![allow(unused)] //-- will let the unused vars be there
#![macro_use] //-- apply the macro_use attribute to the root cause it's an inner attribute and will be effect on all things inside this crate 
use crate::utils;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();






#[near_bindgen] //-- implementing the near_bindgen attribute on Counter struct to compile to wasm
#[derive(Default, BorshDeserialize, BorshSerialize)] //-- the struct needs to implement Default trait which NEAR will use to create the initial state of the contract upon its first usage - need for serde and codec ops - deserialize or map utf8 bytes into this struct from where the contract has called and serialize it to utf8 bytes for compilling it to wasm to run on near blockchain   
pub struct Counter{
    val: i8,
}


#[near_bindgen]
impl Counter{

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