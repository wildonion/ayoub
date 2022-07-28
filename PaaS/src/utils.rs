


use std::sync::{Arc, mpsc::channel as heavy_mpsc};
use std::thread;
use std::sync::mpsc; // NOTE - mpsc means multiple thread can access the Arc<Mutex<T>> but only one of them can mutate the T out of the Arc by locking it
use futures::{executor::block_on, future::{BoxFuture, FutureExt}}; // NOTE - block_on() function will block the current thread to solve the task
use log::info;
use rand::prelude::*;
use crate::constants::*;
use crate::contexts::scheduler::ThreadPool;
use serde::{Serialize, Deserialize};
use borsh::{BorshDeserialize, BorshSerialize};









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

















// ------------------------------ utility methods
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
pub async fn into_box_slice(u8_vector: &Vec<u8>) -> Result<Box<[u8; 4]>, String>{ //-- the return type of this function is either a Box of [u8] slice with 4 bytes (32 bits) or a String of the error
    let to_owned_vec = u8_vector.to_owned(); //-- creating owned vector from borrowed vector by cloning to call into_boxed_slice() method on the vector
    let boxed_slice = to_owned_vec.into_boxed_slice(); //-- converting the collected bytes into a Box slice or array of utf8 bytes - we put it inside the Box cause the size of [u8] is not known at compile time
    let boxed_array: Box<[u8; 4]> = match boxed_slice.try_into() { //-- Boxing u8 with size 4 cause our input number is 32 bits which is 4 packs of 8 bits
        Ok(arr) => return Ok(arr), //-- returning a Box of 4 u8 slice or 4 packs of 8 bits
        Err(o) => return Err(format!("vector length must be 4 but it's {}", o.len())),
    };
}



// -----------------------------------
// handling a recursive async function
// -----------------------------------
// https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html
// NOTE - Future trait is an object safe trait thus we have to Box it with dyn keyword to have kinda a pointer to the heap where the object is allocated in runtime
// NOTE - a recursive `async fn` will always return a Future object which must be rewritten to return a boxed `dyn Future` to prevent infinite size allocation in runtime from heppaneing some kinda maximum recursion depth exceeded prevention process
pub fn async_gen_random_idx(idx: usize) -> BoxFuture<'static, usize>{ // NOTE - pub type BoxFuture<'a, T> = Pin<alloc::boxed::Box<dyn Future<Output = T> + Send + 'a>>
    async move{
        if idx <= CHARSET.len(){
            idx
        } else{
            gen_random_idx(random::<u8>() as usize)
        }
    }.boxed() //-- wrap the future in a Box, pinning it
}





pub fn gen_random_idx(idx: usize) -> usize{
    if idx < CHARSET.len(){
        idx
    } else{
        gen_random_idx(random::<u8>() as usize)
    }
}





pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}











// ------------------------------ heavy computational calculation using async and multithreading design patterns
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
pub fn forward(x_train: Arc<Vec<Vec<f64>>>) -> f64{ //-- without &mut self would be an associated function not a method
        
        
    ////////////////////////////////// multi threading ops
    let thread = thread::spawn(|| async move{ //-- the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) for with type either () or a especific type
        info!("inside the native thread");
        let async_task = tokio::spawn(async move{ //-- spawning async task to solve it on the background using tokio green threads based on its event loop model - 
            info!("inside tokio green thread");
            ////////
            // ....
            ////////
       });
    });
    //////////////////////////////////
    

    let mat = x_train;
    let NTHREADS: usize = 4; // number of threads inside the pool
    let NJOBS: usize = mat.len(); // number of tasks of the process (incoming x_train matrix) to share each one between threads inside the pool
    let pool = ThreadPool::new(NTHREADS);
    let (sender, receiver) = heavy_mpsc();
    let arc_mat = Arc::new(mat);
    let arc_recv = Arc::new(&receiver); //-- take a reference to the receiver to borrow it for putting it inside an Arc
    let mut mult_of_all_sum_cols = 1.0;
    let mut children = Vec::new();
    let future_res = async { //-- we can also use tokio::spawn() to run the async task in the background using tokio event loop and green threads
        for i in 0..NJOBS{ //-- iterating through all the jobs of the process - this can be an infinite loop like waiting for a tcp connection
            let cloned_receiver = Arc::clone(&arc_recv); // can't clone receiver, in order to move it between threads we have to clone it using Arc
            let cloned_sender = sender.clone(); // NOTE - sender can be cloned because it's multiple producer
            let cloned_mat = Arc::clone(&arc_mat);
            children.push(pool.execute(move || { // NOTE - pool.execute() will spawn threads or workers to solve the incoming job inside a free thread - incoming job can be an async task spawned using tokio::spawn() method
                let sum_cols = cloned_mat[0][i] + cloned_mat[1][i] + cloned_mat[2][i];
                cloned_sender.send(sum_cols).unwrap();
            }));
            println!("job {} finished!", i);
        }
        // NOTE - recv() will block the current thread if there are no messages available
        // NOTE - receiver can't be cloned cause it's single consumer
        let ids: Vec<f64> = receiver.iter().take(NJOBS).collect();
        println!("the order that all messages were sent => {:?}", ids);
        ids.into_iter().map(|s_cols| mult_of_all_sum_cols *= s_cols).collect::<Vec<_>>();
        mult_of_all_sum_cols
    };
    let res = block_on(future_res); //-- will block the current thread to run the future to completion
    // let res = future_res.await; //-- .awaiting a future will suspend the current function's execution until the executor has run the future to completion means doesn't block the current thread, allowing other tasks to run if the future is currently unable to make progress
    // let res = join!(future_res); //-- join! only allowed inside `async` functions and blocks and is like .await but can wait for multiple futures concurrently
    println!("multiplication cols sum {:?}", res);
    let loss = 0.3535;
    loss

    
}











// ------------------------------ simd using mpsc channel + tokio + native thread
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------

pub async fn simd<F>(number: u32, ops: F) -> Result<u32, String> where F: Fn(u8) -> u8 + std::marker::Send + 'static + Clone{ //-- in order to move the F between threads it must be bounded to Send trait
        
        
    let threads = 4; //-- the total number of all packs or chunks containing 8 bits which in this case is 4 cause our number is of type u32
    let (sender, receiver) = mpsc::channel::<u8>();
    let big_end_bytes = number.to_be_bytes(); //-- network bytes which is in form utf8 or big endian bytes - since there are 4 chunks of 8 bits in the context of u32 bits there will be 4 chunks of 8 bits each chunk between 0 up to 255 
    let mut index = 0;
    


    while index < big_end_bytes.len(){
        
        info!("chunk {:?} in utf8 format -> [{:?}] at time {:?}", index, big_end_bytes[index], chrono::Local::now().naive_local());
        let cloned_sender = sender.clone();
        let cloned_ops = ops.clone();
        tokio::spawn(async move{ //-- spawning async task to solve it on the background using tokio green threads based on its event loop model
            thread::spawn(move || async move{ //-- the return body of the closure is async block means it'll return a future object (trait Future has implemented for that) with type either () or a especific type and for solving it we have to be inside an async function - in order to capture the variables before spawning scope we have to use move keyword before ||
                let new_chunk = cloned_ops(big_end_bytes[index]);
                info!("\tsender-channel---(chunk {:?})---receiver-channel at time {:?} ", index, chrono::Local::now().naive_local());
                cloned_sender.send(new_chunk).unwrap(); //-- sending new chunk to down side of the channel cause threads must communicate with each other through a mpsc channel to avoid data race condition   
            });
        });
        index+=1

    }

    
    
    info!("collecting all chunks received from the receiver at time {:?}", chrono::Local::now().naive_local());
    let bytes: Vec<u8> = receiver.iter().take(threads).collect(); //-- collecting 4 packs of 8 bits to gather all incoming chunks from the channel
    info!("collected bytes -> {:?} at time {:?}", bytes, chrono::Local::now().naive_local());
    
    

    
    let boxed_array = self::into_box_slice(&bytes).await.unwrap(); //-- converting &Vec<u8> to [u8] with a fixed size
    let result = *boxed_array; //-- dereferencing the box pointer to get the value inside of it 
    let final_res = u32::from_be_bytes(result); //-- will create a u32 number from 4 pack of 8 bits - from_be_bytes() method creates a native endian integer value from its representation as a byte array in big endian
    Ok(final_res) //-- the final results might be different from the input due to the time takes to send the each chunks through the channel and receive them from the receiver thus the order of chunks will not be the same as the input




}












// ------------------------------ testing ownership and borrowing rule
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// https://github.com/wildonion/extrust/blob/4a3e72184ea5159d0ec6d4e8325e481019023b4f/_trash/_garbage.rs#L323
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------

pub struct Pack; //-- we've allocated some space inside the stack for this struct when defining it which has long enough lifetime to initiate an instance from it using struct declaration and return a reference to that instance inside any function 
trait Interface{}

impl Interface for Pack{} //-- is required for return_box_trait() function

fn return_none_trait<T>() -> () where T: Interface{ // NOTE - `T` type must be bound to Interface trait

}

fn return_impl_trait() -> impl Interface { // NOTE - returning impl Trait from function means we're implementing the trait for the object that is returning from the function regardless of its type that we're returning from the function cause compiler will detect the correct type in compile time and implement or bound the trait for that type
    Pack {}
}

fn return_box_trait() -> Box<dyn Interface> { // NOTE - returning Box<dyn Trait> from function means we're returning a struct inside the Box which the trait has implemented for
    Box::new(Pack {})
}

impl Pack{ ////// RETURN BY POINTER EXAMPLE //////


    fn new() -> Self{


        let hello = "Здравствуйте";
        let s = &hello[0..2];
        // every index is the place of an element inside the ram which has 1 byte size which is taken by that element
        // in our case the first element takes 2 bytes thus the index 0 won't return 3 
        // cause place 0 and 1 inside the ram each takes 1 byte and the size of the
        // first element is two bytes thus &hello[0..2] which is index 0 and 1 both returns 3 
        // and we can't have string indices in rust due to this reason!


        ///////////////////////////////////////////// ENUM MATCH TEST
        #[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
        enum Chie{
            Avali(u8),
            Dovomi(String),
            Sevomi,
        }
        
        
        let ine = Chie::Avali(12); //-- the Dovomi variant is never constructed cause we've used the first variant  
        
        match ine{
            Chie::Avali(value) if value == 23 => { //-- matching on the Avali arm if the value was only 23
                println!("u8 eeee");
        
            },
            Chie::Dovomi(value) if value == "wildonion".to_string() => { //-- matching on the Dovomi arm if the value was only "wildonion" string
                println!("stringeeee");
            },
            _ => {
                println!("none of them");
            }
        }

        // --------------- CODEC OPS ON ENUM ---------------
        let encoded = serde_json::to_vec(&Chie::Sevomi); ////// it'll print a vector of utf8 encoded JSON
        let decoded = serde_json::from_slice::<Chie>(&encoded.as_ref().unwrap()); //-- as_ref() returns a reference to the original type

        let encoded_borsh = Chie::Sevomi.try_to_vec().unwrap(); ////// it'll print 2 cause this the third offset in memory
        let decoded_borsh = Chie::try_from_slice(&encoded_borsh).unwrap();

        /////////////////////////////////////////////
        Pack{}
    }
  
    fn ref_struct(num_thread: &u8) -> &Pack{ //-- returning ref from function to a pre allocated data type (not inside the function) Pack struct in our case, is ok
        let instance = Pack::new(); //-- since new() method of the Pack struct will return a new instance of the struct which is allocated on the stack and is owned by the function thus we can't return a reference to it or as a borrowed type
        // &t //-- it's not ok to return a reference to `instance` since `instance` is a local variable which is owned by the current function and its lifetime is valid as long as the function is inside the stack and executing which means after executing the function its lifetime will be dropped
        let instance = &Pack{}; //-- since we're allocating nothing on the stack inside this function thus by creating the instance directly using the the Pack struct and without calling the new() method which is already lives in memory with long enough lifetime we can return a reference to the location of the instance of the pack from the function
        instance //-- it's ok to return a reference to `instance` since the instance does not allocate anything on the stack thus taking a reference to already allocated memory with long enough lifetime is ok since the allocated memory is happened in struct definition line
    }
    
    // NOTE - argument can also be &mut u8
    pub fn ref_str_other_pointer_lifetime(status: &u8) -> &str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we can use the lifetime of the passed in argument, the status in this case which has been passed in by reference from the caller and have a valid lifetime which is generated from the caller scope by the compiler to return the pointer from the function
        let name = "wildonion";
        name //-- name has a lifetime as valid as the passed in status argument lifetime from the caller scope 
    
    }
  
    // NOTE - first param can also be &mut self; a mutable reference to the instance and its fields
    pub fn ref_to_str_other_self_lifetime(&self) -> &str{ //-- in this case we're good to return the pointer from the function or send a copy to the caller's space since we can use the lifetime of the first param which is &self which is a borrowed type of the instance and its fields (since we don't want to lose the lifetime of the created instance from the contract struct after calling each method) and have a valid lifetime (as long as the instance of the type is valid) which is generated from the caller scope by the compiler to return the pointer from the function
        let name = "wildonion";
        name //-- name has a lifetime as valid as the first param lifetime which is a borrowed type of the instance itself and its fields and will borrow the instance when we want to call the instance methods
    }
  
    // NOTE - 'a lifetime has generated from the caller scope by the compiler
    pub fn ref_to_str_specific_lifetime<'a>(status: u8) -> &'a str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we've defined a valid lifetime for the pointer of the return type to return the pointer from the function which &'a str
        let name = "wildonion";
        name //-- name has a lifetime as valid as the generated lifetime from the caller scope by the compiler and will be valid as long as the caller scope is valid
    }

    // NOTE - 'static lifetime will be valid as long as the whole lifetime of the caller scope (it can be the main function which depends on the whole lifetime of the app)
    pub fn ref_to_str_static() -> &'static str{
        let name = "wildonion";
        name //-- name has static lifetime valid as long as the whol lifetime of the caller scope which can be the main function which will be valid as long as the main or the app is valid
    }


}















// ------------------------------ data collision prevention structures 
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
/*
    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------

        

        
        collection 1 keys : {1: "value64", 2: "value53", 3: "value24"}
        collection 2 keys : {1: "anether", 2: "anither", 3: "another"}
        
        when initializing a data structure we have to make sure to give it a unique id, otherwise, it could point to other structure's key-value references;
        above collections will be collided with each other inside the memory since they share the same storage for their keys and have same keys
        to fix this we can allocate a unique storage key for each collection like binding a unique key for each entry that comes into the collection
        and that unique storage key must be built from a utf8 bytes encoded unique indentifire like an enum variant:

        NOTE - the reason we're using enum is because of by encoding each variant using borsh we'll get a unique vector of utf8 bytes array
        
        #[derive(BorshSerialize, BorshDeserialize)]
        pub enum CollectStorageKey{
            CollectionOne,
            CollectionTwo,
        }

        collection 1 storage key : 0 ---- built from the utf8 bytes encoded CollectionOne enum variant (CollectStorageKey::CollectionOne.try_to_vec().unwrap())
        collection 2 storage key : 1 ---- built from the utf8 bytes encoded CollectionTwo enum variant (CollectStorageKey::CollectionTwo.try_to_vec().unwrap())
        
        collection 1 keys : {1: "value64", 2: "value53", 3: "value24"} -> put all the keys inside the created storage key for the first collection like: {0: [1, 2, 3]} or as a unique prefix for the keys: {01: "value64", 02: "value53", 03: "value24"}
        collection 2 keys : {1: "anether", 2: "anither", 3: "another"} -> put all the keys inside the created storage key for the second collection like: {1: [1, 2, 3]} or as a unique prefix for the keys: {11: "anether", 12: "anither", 13: "another"}





        NOTE - by setting a unique storage key for each collection actually we're putting all the keys and entries of that collection inside a unique storage in memory which has a unique key or flag to avoid data collision for each collection's keys
        NOTE - since two different collections might have same key we'll set a prefix key for each collection using enum variant serialized to utf8 to avoid collection collision with same key in their entries, by doing this every collection will have a unique identifier and will be separated from other collection in which a same version of a key exists
        NOTE - every instascne of ByOwnerIdInner, ByNFTContractIdInner and ByNFTTokenTypeInner will have a new memory location address thus we can use it as an storage key since the hash of this key will be different and unique each time due to different memory location address of each instacne which won't be the same even if we create a new instance with a same field each time
        NOTE - enum has an extra size like 8 bytes, a 64 bits pointer which is big enough to store the current vairant address for its tag which tells use which variant we have right now, but rust uses null pointer optimization instead of allocating 8 bytes tag  
        NOTE - null pointer optimization means a reference can never be null such as Option<&T> which is a pinter with 8 bytes length thus rust uses that reference or pointer as the tag with 8 bytes length for the current variant  
        NOTE - none struct variants in Storagekey enum allocates zero byte for the current persistent storage once the tag point to their address at a time
        NOTE - the enum size with zero byte for each variants would be the largest size of its variant + 8 bytes tag which would be 8 bytes in overall
        NOTE - an enum is the size of the maximum of its variants plus a discriminant value to know which variant it is, rounded up to be efficiently aligned, the alignment depends on the platform
        NOTE - an enum size is equals to a variant with largest size + 8 bytes tag
        NOTE - enum size with a single f64 type variant would be 8 bytes and with four f64 variants would be 16 bytes cause one 8 bytes (the tag) wouldn't be enough because there would be no room for the tag
        NOTE - the size of the following enum is 24 (is equals to its largest variant size which belongs to the Text variant) + 8 (the tag size) bytes 
        
        pub enum UserID {
            Number(u64),
            Text(String),
        }
        

    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
*/
#[derive(BorshSerialize)] // NOTE - since UnorderedMap, LookupMap and UnorderedSet each one takes a vector of u8 as their key_prefix argument we have to bound the Storagekey enum to BorshSerialize trait to convert each variant into a vector of u8 using try_to_vec() method of the BorshSerialize trait - all collections (i.e. Vector, Map, Tree, etc) have an unique id which is called the storage key and can be either an encoded enum variant or an encoded string
// -> we've used an enum based storage key for better memory efficiency and avoiding data collision to keeps track of the persistent storage taken by the current collection (one of the following variant). 
// -> data collision could happen by UnorderedMap, LookupMap or UnorderedSet since these hashmap based structure generate a hash from their keys. 
// -> in order not to have a duplicate key entry inside hashmap based structures we can use enum to avoid having some hash collision with two distinct keys.
// -> with enum we can be sure that there will be only one collection (one of the following variant) at a time inside the storage that has been pointed by the enum tag.
// -> hash of the account_id inside the TokensPer* structs is the unique key to use it as the prefix for creating the UnorderedSet to avoid data collision cause every account_id has a unique hash with 256 bits long
pub enum Storagekey{ //-- defining an enum based unique storage key for every our collections to avoid collection collision which might be happened when two different collections share a same storage for their keys on the chain which will face us data collision at runtime
    Sales, ////////---------➔ converting this to vector (Storagekey::Sales.try_to_vec().unwrap()) gives us an array of [0] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key 
    ByOwnerId, ////////---------➔ converting this to vector (Storagekey::ByOwnerId.try_to_vec().unwrap()) gives us an array of [1] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByOwnerIdInner { account_id_hash: [u8; 32] }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id
    ByNFTContractId, ////////---------➔ converting this to vector (Storagekey::ByNFTContractId.try_to_vec().unwrap()) gives us an array of [3] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTContractIdInner { account_id_hash: [u8; 32] }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id
    ByNFTTokenType, ////////---------➔ converting this to vector (Storagekey::ByNFTTokenType.try_to_vec().unwrap()) gives us an array of [5] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTTokenTypeInner { token_type_hash: [u8; 32] }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id
    FTTokenIds, ////////---------➔ converting this to vector (Storagekey::FTTokenIds.try_to_vec().unwrap()) gives us an array of [7] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    StorageDeposits, ////////---------➔ converting this to vector (Storagekey::StorageDeposits.try_to_vec().unwrap()) gives us an array of [8] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
}