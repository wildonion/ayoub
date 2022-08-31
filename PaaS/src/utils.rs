


use std::sync::Mutex;
use std::sync::{Arc, mpsc::channel as heavy_mpsc, mpsc}; use std::time::{SystemTime, UNIX_EPOCH}; // NOTE - mpsc means multiple thread can access the Arc<Mutex<T>> (use Arc::new(&Arc<Mutex<T>>) to clone the arced and mutexed T which T can also be Receiver<T>) but only one of them can mutate the T out of the Arc by locking on the Mutex
use std::{env, thread, fs}; 
use chrono::Utc;
use futures::TryStreamExt;
use futures::{executor::block_on, future::{BoxFuture, FutureExt}}; // NOTE - block_on() function will block the current thread to solve the task
use log::info;
use mongodb::Client;
use mongodb::bson::{self, doc};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use rand::prelude::*;
use routerify::Error;
use crate::{constants::*, schemas};
use crate::contexts::{app, scheduler::ThreadPool};
use serde::{Serialize, Deserialize};
use borsh::{BorshDeserialize, BorshSerialize};
use routerify_multipart::Multipart;











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









#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct UploadFile{
    pub name: String,
    pub time: u64,
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





pub fn string_to_static_str(s: String) -> &'static str { //-- the lifetime of the return str is static and is valid as long as the entire lifetime of the app
    Box::leak(s.into_boxed_str())
}





pub async fn upload_asset(path: &str, mut payload: Multipart<'_>, doc_id: &String){ //-- parsing the incoming file stream into MultipartItem instances - Multipart struct takes a lifetime and we've passed an unnamed lifetime to that
    
    fs::create_dir_all(path).unwrap(); //-- creating the directory which must be contains the file
    let mut filename = "".to_string();
    
    while let Some(mut field) = payload.next_field().await.map_err(|err| Error::wrap(err)).unwrap(){ //-- reading the next field which contains IO stream future object of utf8 bytes of the payload is a mutable process and due to this fact we've defined the payload as a mutable type; we've mapped each incoming utf8 bytes future into an error if there was any error on reading them 
        
        let field_name = field.name(); //-- getting the field's name if provided in "Content-Disposition" header from the client
        let field_file_name = field.file_name(); //-- getting the field's filename if provided in "Content-Disposition" header from the client
        filename = format!("{} - {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), field_file_name.unwrap()); //-- creating the new filename with the server time
        
        let filepath = format!("{}/{}/{}", path, doc_id, sanitize_filename::sanitize(&filename)); //-- creating the new file path with the sanitized filename and the passed in document id
        let buffer = fs::File::create(filepath).unwrap();
        while let Some(chunk) = field.chunk().await.map_err(|err| Error::wrap(err)).unwrap(){ //-- mapping the incoming IO stream of futre object which contains utf8 bytes into a file
            


            
            // TODO - fill the buffer with incoming chunk and write itnto the server hard
            // ...
            // let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();
            // while let Some(chunk) = field.next().await{
            //     let data = chunk.unwrap();
            //     f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            // }
        



        } //-- this field will be dropped in here to get the next field


    }

}




pub async fn set_user_access(username: String, new_access_level: i64, storage: Option<Arc<app::Storage>>) -> Result<schemas::auth::UserInfo, app::Nill<'static>>{ //-- Nill struct requires a lifetime since there is no lifetime has passed to the function we have to use 'static lifetime  

    // NOTE - we can also use clone() method to clone the db instead of using the as_ref() method
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client>
        app::Mode::Off => None, //-- no db is available cause it's off
    };

    ////////////////////////////////// DB Ops

    let update_option = FindOneAndUpdateOptions::builder().return_document(Some(ReturnDocument::After)).build();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let users = app_storage.unwrap().database(&db_name).collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
    match users.find_one_and_update(doc!{"username": username}, doc!{"$set": {"access_level": new_access_level, "updated_at": Some(Utc::now().timestamp())}}, Some(update_option)).await.unwrap(){ //-- finding user based on username to update access_level field to dev access
        Some(user_doc) => Ok(user_doc), 
        None => Err(app::Nill(&[])),
    }

    //////////////////////////////////

}




pub async fn get_random_doc(storage: Option<&Client>) -> Option<schemas::game::RoleInfo>{
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let mut all = vec![];
    let roles = storage.clone().unwrap().database(&db_name).collection::<schemas::game::RoleInfo>("roles");
    let random_record_setup = doc!{"$sample": {"size": 1}};
    let pipeline = vec![random_record_setup];
    match roles.aggregate(pipeline, None).await{
        Ok(mut cursor) => {
            while let Some(random_doc) = cursor.try_next().await.unwrap(){
                let random_role_info = bson::from_document::<schemas::game::RoleInfo>(random_doc).unwrap();
                all.push(random_role_info)
            }
            let role = all[0].clone();
            Some(role)
        },
        Err(e) => None,
    }
}












// ------------------------------ heavy computational calculation using async and multithreading design patterns
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------------------------------------------
pub fn forward(x_train: Arc<Vec<Vec<f64>>>) -> f64{ //-- without &mut self would be an associated function not a method
    

    /*  


        ➔ types bounded to Sync and Send:
            Arc
            Mutex
            RwLock
        ➔ types not bounded to Sync and Send:
            Rc
            RefCell
            Cell



       ➔ in order to move the T between threads T must be bounded to Send and if it wasn't bound to Sync we have to clone it to move it between threads 
          and if it wasn't bound to Clone trait we must put it inside the Arc and to change it inside other thread we must put the Arc-ed type inside 
          the Mutex like mutating the Receiver<T> content inside other threads which must be in form Arc<Mutex<Receiver<T>>> since the Receiver is 
          not bounded to Sync (cause &Receiver is not bouned to Send cause there is no clone trait implemented for the Receiver thus it can't be copy 
          thus we can't have &Receiver and can't be clone) therefore we can't move it between threads due to the fact that the type T (Receiver in our case) 
          must be used by only one consumer or thread at a time by blocking threads waiting for the lock to become available.
       ➔ based on the rust ownership and borrowing rule we can have multiple immutable references to a type but only one mutable pointer at a time must be exists
          and due to this rule we can have a safe concurrency in such a way that multiple threads can be the owner of a type to read its content but only on of them
          can mutate the content of the type (like reading from the receiver which is a mutable operation) therefore we can have mpsc message passing channel based on this rule
          the sender of the channel is Send but not Sync and can be cloned to send data between threads for reading (multiple producer) but the receiver is not Send and Sync and can't be cloned 
          since it can't be move or transfer to other threads cause the reading process from the receiver is a mutable operation and we can't have multiple mutable operations or references 
          at the same time with multiple threads thus only one thread at a time can do this job.
       ➔ since the reading from the receiver is a mutable process and it can't be cloned thus in order to read from it inside multiple thread we have to write 
          the receiver in form of Arc<Mutex<Receiver<T>>> and lock on the receiver to read the content of what we're receiving.   



        -- shareable rules : data which are Send + Sync + 'static must be share and trasferred between threads using mpsc channel
        -- in rust everything is all about having a type and size thus everything must be generic and we have to borrowing them using & to share them between other scopes like threads and functions a shareable data means we're sharing its references which it can be copied or cloned and safe to Send and mutate between threads
        -- Arc will be used instead of Rc in multi threading to avoid data races and is Send means all its references can be shared between threads and is an atomic reference to a type
        -- if &T is Send then T can be also Sync thus in order to share a data between threads safely the type must be bounded to Send + Sync + 'static means it must be cloneable or shareable between threads means we can simply borrow it to move it between threads and Sync with other threads to avoid mutating it by multiple threads at the same time
        -- if there is no possibility of undefined behavior like data races when passing &T between threads means &T must be Send then T is alos Sync and &mut T is Sync if T is Sync
        -- data which is utf8 encoded using borsh or serde to share a reference of it (by borrowing it) between threads using mpsc must be : Send + Sync + 'static + Unpin means it must be inside Arc<Mutex<Data>>
        -- if a type is not Send + Sync it means we can't move its references between threads safely and we have to put it inside Arc since &Arc<T> is Send thus Arc<T> is also Sync
        -- a type might be mutated by other threads thus we have to put it inside Mutex or RwLock to avoid data races means that only one thread can mutate the state of a type
        -- instead of moving types into the thread we can borrow them using Arc to have them outside the threads
        -- based on mpsc rust has defined the rule which says multiple immutable can be inside a scope but only one of them can be mutable
        -- in order to share data (T must have shareable rules) between threads we have to use mpsc channel 
        -- Send is the access of sharing between threads, Sync is safe to transfer and static means the type must have static lifetime across threads and .awaits
        -- share data between routers and threads using .data() of routerify Router and to do that the passed in closure of the thread must be static + send and sync to send between threads safely we can't just simply borrow the data using & to pass them between threads (since the race condition might be happened) since the type must be send + sync and 'static to be shared between threads safely if it's not send and sync we can put it inside the Arc<Mutex<T>> to make it cloneable and borrow it mutably to mutate its content by locking on it inside a free thread, if other threads don't want to mutate it we can just put it inside Arc<T> to be just cloneable 
        -- share reference or share access means multiple threads can read and access a resource or a type but only on of them can mutate it and the channel for this task is the mpsc
        -- the type that wants to be sent between threads must be Send but not Sync necessarily like sender which is not Sync but it's Send and receiver is not Sync and Send
        -- it's better not to pass the receiver between threads due to the rule of mpsc since we can't mutate a data simply inside a thread while others are reading it we have to block that thread that wants to mutate the type using Mutex
        -- passing data between threads is done using mpsc channel which multiple threads can own a resource immutable referece but only on of them can mutate that resource at a time
        -- to pass data between thread the type must cloneable and sender must be cloned since inside a thread all env vars before that are moved to its scope.
        -- in order to mutate a type inside a thread the type must be inside Mutex since the receiver can't be referenced by multiple threads at a time thus is a single consumer means that it can't be cloned and moved to other threads 
        -- Send means that a type is safe to move from one thread to another
        -- Sync makes the type safe (&T nmust be Send) to access shared reference across threads at the same time 
        -- Clone trait is implemented for the mpsc sender and is bounded to Send but not Sync and due to this reason we have to clone it in order we can have it in multiple threads (multi producer)
        -- Clone trait is not implemented for the mpsc receiver and we must put it inside Arc also is not Sync means it can't be referenced by multiple threads at the same time due to the fact that only one thread can mutate its content at a time (single consumer) thus we have to put it inside a Mutex
        -- in order to pass the receiver between threads safely and mutate its content by locking on it we must put the receiver inside Arc and Mutex like let shareable_receiver = Arc<Mutex<Receiver<T>>> then clone it using Arc::new(&shareable_receiver) or shareable_receiver.clone()
        -- recv() will block the current thread if there are no messages available


    */
    
    
    let mat = x_train; //-- the data that we want to do some heavy computational on it
    let NTHREADS: usize = 4; //-- number of threads inside the pool
    let NJOBS: usize = mat.len(); //-- number of tasks of the process (incoming x_train matrix) to share each one between threads inside the pool
    let pool = ThreadPool::new(NTHREADS);
    let (sender, receiver) = heavy_mpsc::<f64>();


    let mutexed_receiver = Mutex::new(receiver); //-- putting the &receiver in its borrowed form inside the Mutex to get its data by locking on it inside other threads since the Sync is not implemented for the receiver and in order to get its data inside other threads we have to make cloneable using Arc and some kina syncable using Mutext
    let arced_mutexed_receiver = Arc::new(mutexed_receiver); //-- putting the &mutexed_receiver in its borrowed form inside the Arc
    pub static mut MULT_OF_ALL_SUM: f64 = 1.0;
    let mut mult_of_all_sum: &'static f64 = &1.0;
    let mut children = Vec::new();

    
    let future_res = async { //-- we can also use tokio::spawn() to run the async task in the background using tokio event loop and green threads
        
        for i in 0..NJOBS{ //-- iterating through all the jobs of the process - this can be an infinite loop like waiting for a tcp connection
            let cloned_arced_mutexed_receiver = Arc::clone(&arced_mutexed_receiver); //-- in order to move the receiver between threads we must have it in Arc<Mutex<Receiver<T>>> form and clone the &arced_mutexed_receiver which is the borrowed form of the arced and mutexed of the receiver or we can have arced_mutexed_receiver.clone()
            let cloned_sender = sender.clone(); //-- cloning the sender since it's multiple producer and Clone trait is implemented for that
            let cloned_mat = mat.clone();
            children.push(
                pool.execute(move || { //-- pool.execute() will spawn threads or workers to solve the incoming job inside a free thread - incoming job can be an async task spawned using tokio::spawn() method
                    let sum_cols = cloned_mat[0][i] + cloned_mat[1][i] + cloned_mat[2][i];
                    cloned_sender.send(sum_cols).unwrap();
                })
            );
            
            info!("job {} finished!", i);
            
            /* ----------------------------------------------------------------------------------- */
            /* -------- receiving inside another native and tokio threads inside the loop -------- */ 
            /* ----------------------------------------------------------------------------------- */
            // thread::spawn(move || loop{});
            thread::spawn(|| async move{ //-- the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) for with type either () or a especific type
                tokio::spawn(async move{ //-- spawning async task to solve it on the background using tokio green threads based on its event loop model
                    while let Ok(data) = cloned_arced_mutexed_receiver.lock().unwrap().recv(){
                        /* 
                            -----------------------------------------------------------------------------------
                                          --- the reason that MULT_OF_ALL_SUM must be static ---

                            in this situation we shouldn't mutate any type inside here if the type is not
                            static and doesn't have a valid lifetime across threads.

                            one of the most popular cases of a T: 'static bound is std::thread::spawn
                            the reason there is that the closure and its return value need to be sent between threads, 
                            escaping their call-stack which is why they cannot contain any non-'static references 
                            since these could become invalidated and no longer available in the other thread in the mean-time.

                            if we don't want to send the data from a thread to another one we have to make static to have 
                            valid lifetime across threads also mutating the static types directly is unsafe!

                            due to the single consumer rule only on thread can mutate the received job or the task or the data
                            at a time in order to prevent data racing we've put the Arced (since it's not cloneable due to the single consumer rule) 
                            receiver inside the Mutex to lock on it and change the content of what it has received cause we want 
                            to mutate the data of the receiver inside other threads.
                            -----------------------------------------------------------------------------------
                        */
                        
                        // *mult_of_all_sum *= data; // ERROR - can't deref the mult_of_all_sum since its deref doesn't live long enough since its reference or its borrowed type is static not its deref 
                        unsafe {MULT_OF_ALL_SUM *= data;} //-- mutating the data that we've just received - mutating static types needs unsafe block
                    }
               });
            });
            /* ----------------------------------------------------------------------------------- */
        }
        
        unsafe{MULT_OF_ALL_SUM} //-- since MULT_OF_ALL_SUM has mutated thus we have to return it from an unsafe block

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

















// ---------------------------------------- interfaces
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------

pub struct OTPSuccess;
pub struct OTPErr;
pub struct PhoneNumber;
pub struct Auth;

pub trait Otp{

    type Message;

    fn send_code(&mut self, recipient: PhoneNumber, message: Self::Message) -> Result<OTPSuccess, OTPErr>;

}

impl Otp for Auth{

    type Message = String;
    
    fn send_code(&mut self, recipient: PhoneNumber, message: Self::Message) -> Result<OTPSuccess, OTPErr>{

        todo!()

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
        NOTE - an enum size is equals to a variant with largest size + 8 bytes tag (there is only one 8 byte tag required since only one variant will be available at the same time)
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
    ByOwnerIdInner { account_id_hash: [u8; 32] }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id which must be serialize to vector of utf8 bytes to use that vector as the prefix key
    ByNFTContractId, ////////---------➔ converting this to vector (Storagekey::ByNFTContractId.try_to_vec().unwrap()) gives us an array of [3] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTContractIdInner { account_id_hash: [u8; 2] }, //-- 2 bytes or 256 bits (cause it's an array of 2 elements of type u8 which is 2 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id which must be serialize to vector of utf8 bytes to use that vector as the prefix key
    ByNFTTokenType, ////////---------➔ converting this to vector (Storagekey::ByNFTTokenType.try_to_vec().unwrap()) gives us an array of [5] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    ByNFTTokenTypeInner { token_type_hash: [u8; 32] }, //-- 32 bytes or 256 bits (cause it's an array of 32 elements of type u8 which is 32 elements with 1 byte size) of the hash which will be 64 chars in hex which is the account_id length; use this to cover the prefix of the collection storage key based on a struct which contains the hash of the account_id which must be serialize to vector of utf8 bytes to use that vector as the prefix key
    FTTokenIds, ////////---------➔ converting this to vector (Storagekey::FTTokenIds.try_to_vec().unwrap()) gives us an array of [7] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    StorageDeposits, ////////---------➔ converting this to vector (Storagekey::StorageDeposits.try_to_vec().unwrap()) gives us an array of [8] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
    Collection, ////////---------➔ converting this to vector (Storagekey::Collection.try_to_vec().unwrap()) gives us an array of [9] which is the utf8 bytes encoded version of the current variant (the offset in memory) that can be used as a unique storage key for the collection prefix key
}














// ------------------------------ utility macros
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

// https://doc.rust-lang.org/reference/procedural-macros.html
// TODO - build function like macro like query!() and custom inner and outter trait like proc macro attributes and derive like; on structs, fields, modules and functions like #[near_bindgen] and #[borsh_skip] proc macro attribute, #[custom(token_stream)] and #[derive(Clone)] style 
// TODO - write proc macro attributes and derives with TokenStream arg using proc_macro2 crate and proc-macro = true flag inside the lib.rs file by using #[proc_macro], #[proc_macro_attribute] and #[proc_macro_derive] attributes  
// TODO - a proc macro attribute to convert a trait into a module and its methods into static methods of that module and add extra args like the ones for nft_on_transfer() and nft_on_approve() methods when the user is implementing these methods
// TODO - VM, interpreter and #[wowasm] proc macro attribute to write smart contracts with wo syntax to compile to wasm to run on near
// TODO - create a new language with macro based syntax
// NOTE - we can use [], {} or () to call macros
// NOTE - #[derive(Trait, SomeMacro)] bounds a struct to a trait or a macro
// NOTE - #[..] applies an attribute to the thing after it (struct, struct fields or crate) and  #![..] applies an attribute to the containing thing or crate
// ...


/*

    item      ➔ an Item | an item, like a function, struct, module, etc.
    block     ➔ a BlockExpression | a block (i.e. a block of statements and/or an expression, surrounded by braces)
    stmt      ➔ a Statement without the trailing semicolon (except for item statements that require semicolons)
    pat_param ➔ a PatternNoTopAlt
    pat       ➔ at least any PatternNoTopAlt, and possibly more depending on edition
    expr      ➔ an Expression
    ty        ➔ a Type
    ident     ➔ an IDENTIFIER_OR_KEYWORD or RAW_IDENTIFIER
    path      ➔ a TypePath style path | a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
    tt        ➔ a TokenTree (a single token or tokens in matching delimiters (), [], or {})
    meta      ➔ an Attr, the contents of an attribute | a meta item; the things that go inside #[...] and #![...] attributes
    lifetime  ➔ a LIFETIME_TOKEN
    vis       ➔ a possibly empty Visibility qualifier
    literal   ➔ matches -?LiteralExpression


*/


#[macro_use]
pub mod macros{

    pub fn even(x: i32) -> bool{
        x%2 == 0
    }
    
    pub fn odd(x: i32) -> bool{
        x%2 != 0
    }
    
    #[macro_export]
    macro_rules! list {
        ($id1:ident | $id2:ident <- [$start:expr; $end:expr], $cond:expr) => { //-- the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc
            { //.... code block to return vec since using let statements must be inside {} block
                let mut vec = Vec::new();
                for num in $start..$end + 1{
                    if $cond(num){
                        vec.push(num);
                    }
                }
                vec
            } //....
        };
    }
    //////
    /// let evens = list![x | x <- [1; 10], even];
    //////
    

    #[macro_export]
    macro_rules! dict {
        ($($key:expr => $val:expr)*) => { //-- if this pattern matches the input the following code will be executed - * means we can pass more than one key => value statement
            { //.... code block to return vec since using let statements must be inside {} block
                use std::collections::HashMap;
                let mut map = HashMap::new();
                $(
                    map.insert($key, $value);
                )* //-- * means we're inserting multiple key => value statement inside the map 
                map
            } //....
        };
    }
    //////
    /// let d = dict!{"wildonion" => 1, "another_wildonion" => 2};
    //////
    
    #[macro_export]
    macro_rules! exam {
        ($l:expr; and $r:expr) => { //-- logical and match 
            $crate::macros::even(); //-- calling even() function which is inside the macros module
            println!("{}", $l && $r);
        };
    
        ($l:expr; or $r:expr) => { //-- logical or match 
            println!("{}", $l || $r);
        };
    }
    //////
    /// exam!(1 == 2; and 3 == 2+1)
    /// exam!(1 == 2; or 3 == 2+1)
    //////
    
    
    #[macro_export]
    macro_rules! wowasm {
        ($iden:ident, $ty: tt) => {
            pub struct $iden(pub $ty);
            impl Default for $iden{
                fn default() -> Self{
                    todo!()
                }
            }  
        };
    
        ($func_name:ident) => {
            fn $func_name(){
                println!("you've just called {:?}()", stringify!($func_name));
            }
        }
    }
    
    
    #[macro_export]
    macro_rules! query { // NOTE - this is a macro with multiple syntax support and if any pattern matches with the caller pattern, then the code block of that pattern will be emitted
        
        ( $value_0:expr, $value_1:expr, $value_2:expr ) => { //-- passing multiple object syntax
            // ...
        };
    
        ( $($name:expr => $value:expr)* ) => { //-- passing multiple key => value syntax 
            // ...
    
        };
    
    }
    
    
    #[macro_export]
    macro_rules! log {
        ($arg:tt) => { //-- passing single String message 
            $crate::env::log($arg.as_bytes()) //-- log function only accepts utf8 bytes
        };
        ($($arg:tt)*) => { //-- passing multiple String messages 
            $crate::env::log(format!($($arg)*).as_bytes()) //-- log function only accepts utf8 bytes
        };
    }
    
    
    #[macro_export]
    macro_rules! impl_engine_constructor {
        ($( $new:ident: [ $( $pos:expr ),* ] anchored at $anchor:expr; )*) => { //-- the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc 
            $(
                pub fn $new() -> Self{
                    Self{
                        positions: [$( $pos ),*].into_iter().collect(),
                        anchor: $anchor,
                    }
                }
            )* //-- * means defining function for every new Pos
        };
    }
    
    
    // #[derive(Debug, Clone)]
    // pub struct Shape{
    //     typ: &'static str,
    //     positions: HashSet<Pos>,
    //     anchor: Pos,
    // }
    
    
    // #[derive(Debug, Clone, Copy)]
    // pub struct Pos(pub i32, pub i32);
    
    
    
    // impl Shape {
    //     impl_engine_constructor! {
    //       new_i "🟦": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
    //       new_o "🟨": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
    //       new_t "🟫": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(1, 0);
    //       new_j "🟪": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
    //       new_l "🟧": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
    //       new_s "🟩": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
    //       new_z "🟥": [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
    //     }
    // }
}