


use std::sync::{Arc, mpsc::channel as heavy_mpsc};
use std::thread;
use std::sync::mpsc; // NOTE - mpsc means multiple thread can access the Arc<Mutex<T>> but only one of them can mutate the T out of the Arc by locking it
use futures::{executor::block_on, future::{BoxFuture, FutureExt}}; // NOTE - block_on() function will block the current thread to solve the task
use log::info;
use rand::prelude::*;
use crate::constants::*;
use crate::contexts::scheduler::ThreadPool;









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













// ------------------------------ into box slice method
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
    // TODO - use other Box methods
    // ...
    Box::leak(s.into_boxed_str())
}





pub struct Pack;
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
        Pack{}
    }
  
    fn run_pool(num_thread: &u8) -> &Pack{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we can use the lifetime of the passed in argument, the status in this case which has been passed in by reference from the caller and have a valid lifetime which is generated from the caller scope by the compiler to return the pointer from the function
        let instance = Pack::new(); //-- since new() method of the Pack struct will return a new instance of the struct which will be owned by the function thus we can't return a reference to it or as a borrowed type
        // &t //-- it's not ok to return a reference to `instance` since `instance` is a local variable which is owned by the current function and its lifetime is valid as long as the function is inside the stack and executing which means after executing the function its lifetime will be dropped
        let instance = &Pack{}; //-- since 
        instance //-- it's ok to return a reference to `instance` since  
    }
    
    // NOTE - argument can also be &mut u8
    pub fn cons(status: &u8) -> &str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we can use the lifetime of the passed in argument, the status in this case which has been passed in by reference from the caller and have a valid lifetime which is generated from the caller scope by the compiler to return the pointer from the function
        let name = "wildonion";
        name //-- name has a lifetime as valid as the passed in status argument lifetime from the caller scope 
    
    }
  
    // NOTE - first param can also be &mut self; a mutable reference to the instance and its fields
    pub fn none_cons(&self) -> &str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we can use the lifetime of the first param which is &self which is a borrowed type of the instance and its fields (since we don't want to lose the lifetime of the created instance from the contract struct after calling each method) and have a valid lifetime which is generated from the caller scope by the compiler to return the pointer from the function
        let name = "wildonion";
        name //-- name has a lifetime as valid as the first param lifetime which is a borrowed type of the instance itself and its fields and will borrow the instance when we want to call the instance methods
    }
  
    // NOTE - 'a lifetime has generated from the caller scope by the compiler
    pub fn cons_valid_lifetime<'a>() -> &'a str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we've defined a valid lifetime for the pointer of the return type to return the pointer from the function which &'a str
        let name = "wildonion";
        name //-- name has a lifetime as valid as the generated lifetime from the caller scope by the compiler and will be valid as long as the caller scope is valid
    }


}