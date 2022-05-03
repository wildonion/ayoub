


use std::thread;
use std::sync::mpsc;
use futures::future::{BoxFuture, FutureExt};
use log::info;
use crate::constants::*;
use rand::prelude::*;







pub mod jwt{

    use std::env;
    use chrono::Utc;
    use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, TokenData};
    use serde::{Serialize, Deserialize};
    use mongodb::bson::oid::ObjectId;



    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims{
        pub _id: Option<ObjectId>, //-- mongodb object id
        pub username: String,
        pub access_level: u8,
        pub exp: i64, //-- expiration timestamp
        pub iat: i64, //-- issued timestamp
    }



    pub async fn construct(payload: Claims) -> Result<String, jsonwebtoken::errors::Error>{
        let encoding_key = env::var("SECRET_KEY").expect("⚠️ no secret key variable set");
        let token = encode(&Header::new(Algorithm::HS512), &payload, &EncodingKey::from_secret(encoding_key.as_bytes()));
        token
    }

    pub async fn deconstruct(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error>{
        let encoding_key = env::var("SECRET_KEY").expect("⚠️ no secret key variable set");
        let decoded_token = decode::<Claims>(token, &DecodingKey::from_secret(encoding_key.as_bytes()), &Validation::new(Algorithm::HS512));
        decoded_token
    }

    pub async fn gen_times() -> (i64, i64){
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
        let exp_time = now + env::var("JWT_EXPIRATION").expect("⚠️ found no jwt expiration time").parse::<i64>().unwrap();
        (now, exp_time)
    }

}







pub mod user{

    use crate::schemas;
    use mongodb::{Client, bson::{self, doc, oid::ObjectId}}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file


    pub async fn exists(storage: Option<&Client>, user_id: Option<ObjectId>, username: String, access_level: u8) -> bool{

        ////////////////////////////////// DB Ops

        let serialized_access_level = bson::to_bson(&access_level).unwrap(); //-- we have to serialize the access_level to BSON Document object in order to find a user with this info cause mongodb can't do serde ops on raw u8
        let users = storage.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
        match users.find_one(doc!{"_id": user_id, "username": username, "access_level": serialized_access_level}, None).await.unwrap(){ //-- finding user based on username
            Some(user_doc) => true, 
            None => false,
        }

        //////////////////////////////////
 
    }

}








// ------------------------------ using mpsc channel + tokio + native thread
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------

pub async fn simd<F>(number: u32, ops: F) -> Result<u32, String> where F: Fn(u8) -> u8 + std::marker::Send + 'static + Clone{ //-- in order to move the F between threads it must be bounded to Send trait
        
        
    let threads = 4; //-- the total number of all packs or chunks containing 8 bits which in this case is 4 cause our number is of type u32
    let (sender, receiver) = mpsc::channel::<u8>();
    let big_end_bytes = number.to_be_bytes(); //-- network bytes - since there are 4 chunks of 8 bits in the context of u32 bits there will be 4 chunks of 8 bits each chunk between 0 up to 255 
    let mut index = 0;
    


    while index < big_end_bytes.len(){
        
        info!("chunk {:?} in utf8 format -> [{:?}] at time {:?}", index, big_end_bytes[index], chrono::Local::now().naive_local());
        let cloned_sender = sender.clone();
        let cloned_ops = ops.clone();
        tokio::spawn(async move{
            thread::spawn(move || async move{ //-- the return body of the closure is async and for solving it we have to be in an async function - in order to capture the variables before spawning scope we have to use move keyword before ||
                let new_chunk = cloned_ops(big_end_bytes[index]);
                info!("\tsender-channel---(chunk {:?})---receiver-channel at time {:?} ", index, chrono::Local::now().naive_local());
                cloned_sender.send(new_chunk).unwrap();
            });
        });
        index+=1

    }

    
    
    info!("collecting all chunks received from the receiver at time {:?}", chrono::Local::now().naive_local());
    let bytes: Vec<u8> = receiver.iter().take(threads).collect(); //-- collecting 4 packs of 8 bits to gather all incoming chunks from the channel
    info!("collected bytes -> {:?} at time {:?}", bytes, chrono::Local::now().naive_local());
    let boxed_slice = bytes.into_boxed_slice(); //-- converting the collected bytes into a Box slice or array of utf8 bytes - we put it inside the Box cause the size of [u8] is not known at compile time
    let boxed_array: Box<[u8; 4]> = match boxed_slice.try_into() { //-- Boxing u8 with size 4 cause our input number is 32 bits which is 4 pack of 8 bits 
        Ok(arr) => arr,
        Err(o) => return Err(format!("vector length must be 4 but it's {}", o.len())),
    };
    
    
    
    let result = *boxed_array; //-- dereferencing the box pointer to get the value inside of it 
    let final_res = u32::from_be_bytes(result); //-- will create a u32 number from 4 pack of 8 bits 
    Ok(final_res) //-- the final results might be different from the input due to the time takes to send the each chunks through the channel and receive them from the receiver thus the order of chunks will not be the same as the input


}





// -----------------------------------
// handling a recursive async function
// -----------------------------------
// https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
// NOTE - Future trait is an object safe trait thus we have to Box it with dyn keyword to have kinda a pointer to the heap where the object is allocated in runtime
// NOTE - a recursive `async fn` will always return a Future object which must be rewritten to return a boxed `dyn Future` to prevent infinite size allocation in runtime from heppaneing some kinda maximum recursion depth exceeded prevention process
// pub fn gen_random_idx(idx: usize) -> BoxFuture<'static, usize>{ // NOTE - pub type BoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>
//     async move{
//         if idx <= CHARSET.len(){
//             idx
//         } else{
//             gen_random_idx(random::<u8>() as usize).await
//         }
//     }.boxed() //-- wrap the future in a Box, pinning it
// }





pub fn gen_random_idx(idx: usize) -> usize{
    if idx < CHARSET.len(){
        idx
    } else{
        gen_random_idx(random::<u8>() as usize)
    }
}



pub fn string_to_static_str(s: String) -> &'static str {
    // TODO - use other Box methods
    // ...
    Box::leak(s.into_boxed_str())
}





struct Struct;
trait Interface{}

impl Interface for Struct{} //-- is required for return_box_trait() function

fn return_impl_trait() -> impl Interface { // NOTE - returning impl Trait from function means we're implementing the trait for the object that is returning from the function regardless of its type that we're returning from the function cause compiler will detect the correct type in compile time and implement or bound the trait for that type
    Struct {}
}

fn return_box_trait() -> Box<dyn Interface> { // NOTE - returning Box<dyn Trait> from function means we're returning a struct inside the Box which the trait has implemented for
    Box::new(Struct {})
}



#[macro_export]
macro_rules! res {
    
    ( $message:expr, $status:expr, $data:expr ) => { //-- passing multiple object syntax
        use hyper::{header, StatusCode, Body};
        // ...
    
    };

    ( $($name:expr => $value:expr),* ) => { //-- passing multiple key => value syntax 
        use hyper::{header, StatusCode, Body};
        // ...

    };

}