


/*


    ## About the `simd` function

    [Question](https://quera.org/problemset/113613/)


    ### Inputs

    * An operation function
    * u32 bits number

    ### Output

    * u32 bits number


    ### Sample Input

    * _heavy_func_
    * _3985935_

    ### Sample Output on Equal Condition

    ```console
    INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T18:19:47.883156
    INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T18:19:47.885159800
    INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T18:19:47.885159800
    INFO  simd  >  --------Doing some heavy operation on chunk [0]
    INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T18:19:47.885159800
    INFO  simd  >  --------Doing some heavy operation on chunk [60]
    INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T18:19:47.885159800
    INFO  simd  >  --------Doing some heavy operation on chunk [210]
    INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T18:19:47.886155
    INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T18:19:47.886155
    INFO  simd  >  --------Doing some heavy operation on chunk [15]
    INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T18:19:47.886155
    INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T18:19:47.887157100
    INFO  utils > collected bytes -> [0, 60, 210, 15] at time 2022-03-16T18:19:47.887157100
    INFO  simd  > ::::: the result is 3985935 - [it might be different from the input] - | cost : 4.0779
    ```

    ### Sample Output on Unequal Condition

    ```console
    INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T18:20:57.775299
    INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T18:20:57.776326200
    INFO  simd  >  --------Doing some heavy operation on chunk [0]
    INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T18:20:57.779358200
    INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T18:20:57.780341
    INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T18:20:57.780341
    INFO  simd  >  --------Doing some heavy operation on chunk [60]
    INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T18:20:57.783330100
    INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T18:20:57.782328700
    INFO  simd  >  --------Doing some heavy operation on chunk [15]
    INFO  simd  >  --------Doing some heavy operation on chunk [210]
    INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T18:20:57.787324900
    INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T18:20:57.788324300
    INFO  utils > collected bytes -> [0, 60, 15, 210] at time 2022-03-16T18:20:57.790324800
    INFO  simd  > ::::: the result is 3936210 - [it might be different from the input] - | cost : 15.9839
    ```

    ### The Beauty of Concurrency!

    **NOTE** - Due to the time which takes to send and receive each chunks inside threads through the `mpsc` channel asyncly, the result might be different on each run and it depends on the system, but here at first run both input and the result got into an equality condition.


*/



use std::{sync::mpsc, thread, time::Instant}; // NOTE - mpsc means multiple thread can access the Arc<Mutex<T>> but only one of them can mutate the T
use crate::*;  // load all defined crates, structs and functions from the root crate which is lib.rs in our case













// ------------------------------ message passing & multi threading (mpsc &  native threads)
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// NOTE - since can't compile socket in lib (wasm and bpf) mode, contracts can't interact with their outside worlds thus we can't have whether tokio or any web framework to handle async functions
pub fn simd<F>(number: u32, ops: F) -> Result<u32, String> where F: Fn(u8) -> u8 + std::marker::Send + 'static + Clone{ //-- in order to move the F between threads it must be bounded to Send trait
        
        
    let threads = 4; //-- the total number of all packs or chunks containing 8 bits which in this case is 4 cause our number is of type u32
    let (sender, receiver) = mpsc::channel::<u8>();
    let big_end_bytes = number.to_be_bytes(); //-- network bytes - since there are 4 chunks of 8 bits in the context of u32 bits there will be 4 chunks of 8 bits each chunk between 0 up to 255 
    let vector_of_big_end_bytes = Vec::<u8>::from(big_end_bytes); //-- converting [u8] bytes to Vec<u8> using from() methods of the From trait implemented for the Vec type
    let mut index = 0;
    


    while index < big_end_bytes.len(){
        
        let log_message = format!("chunk {:?} in utf8 format -> [{:?}] at time {:?}", index, big_end_bytes[index], chrono::Local::now().naive_local());
        env::log(log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
        let cloned_sender = sender.clone();
        let cloned_ops = ops.clone();
        thread::spawn(move ||{ //-- spawning a native thread - can't have async body here (like spawning an async task inside the tokio green threads) to get the result out from the thread asyncly cause we are not inside an async function to solve the future inside the body of closure of the thread (or the async task of the tokio) by joining on the spawned thread 
            let new_chunk = cloned_ops(big_end_bytes[index]);
            let new_chunk_log_message = format!("\tsender-channel---(chunk {:?})---receiver-channel at time {:?} ", index, chrono::Local::now().naive_local());
            env::log(new_chunk_log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
            cloned_sender.send(new_chunk).unwrap();
        });
        index+=1

    }

    
    
    let start = Instant::now();
    let collecting_chunk_log_message = format!("collecting all chunks received from the receiver at time {:?}", chrono::Local::now().naive_local());
    env::log(collecting_chunk_log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
    let bytes: Vec<u8> = receiver.iter().take(threads).collect(); //-- collecting 4 packs of 8 bits to gather all incoming chunks from the channel
    let end = Instant::now();
    let delta = end.duration_since(start);
    let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
    let collected_chunk_log_message = format!("collected bytes -> {:?} at time {:?}", bytes, chrono::Local::now().naive_local());
    env::log(collected_chunk_log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!() 
    let boxed_slice = bytes.into_boxed_slice(); //-- converting the collected bytes into a Box slice or array of utf8 bytes - we put it inside the Box cause the size of [u8] is not known at compile time
    let boxed_array: Box<[u8; 4]> = match boxed_slice.try_into() { 
        Ok(arr) => arr,
        Err(o) => return Err(format!("vector length must be 4 but it's {}", o.len())),
    };
    
    
    
    let result = *boxed_array; //-- dereferencing the box pointer to get the value inside of it 
    let final_res = u32::from_be_bytes(result); //-- will create a u32 number from 4 pack of 8 bits 
    let result_chunk_log_message = format!("::::: the result is {:?} - [it might be different from the input] - | cost : {:?}\n\n", final_res, delta_ms);
    env::log(result_chunk_log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!() 
    Ok(final_res) //-- the final results might be different from the input due to the time takes to send the each chunks through the channel and receive them from the receiver thus the order of chunks will not be the same as the input


}









// ------------------------------ helper function to run simd ops
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
pub fn simd_ops(){ //-- this function can't be invoked directly on the blockchain or nodes; it can be called from an invoked function
    

    env::log("Overflow warning".as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
    
    
    let heavy_func = |chunk: u8| {
        let log_message = format!("\t--------Doing some heavy operation on chunk [{:?}]", chunk);
        env::log(log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
        chunk
    };

    let simd_input = 2874843;
    
    let start = Instant::now();
    match simd(simd_input, heavy_func){
        Ok(result) => {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32; 
            // assert_eq!(3985935_u32, result); //-- it'll panic on not equal condition
            let log_message = format!("::::: the result is {:?} - [it might be different from the input] - | cost : {:?}\n\n", result, delta_ms); 
            env::log(log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
        },
        Err(e) => {
            let log_message = format!("::::: error in reading chunk caused by {:?}", e); 
            env::log(log_message.as_bytes()); //-- converting the log message to ut8 bytes cause env::log() has lower runtime cost than the println!() and info!()
        },
    }

}






/*
    ---------------------------------

        NOTE - none struct variant in Storagekey enum allocates zero byte for the current persistent storage once the tag point to its address 
        NOTE - tag is a 64 bits or 8 bytes pointer and is big enough to store the current vairant address
        NOTE - an enum is the size of the maximum of its variants plus a discriminant value to know which variant it is, rounded up to be efficiently aligned, the alignment depends on the platform
        NOTE - an enum and its tag size with one variant is equals to the size of that variant
        NOTE - an enum size wit more than one variant is equals to a variant with largest size + 8 bytes tag cause 
        NOTE - enum size with a single f64 type variant would be 8 bytes and with four f64 variants would be 16 bytes cause one 8 bytes wouldn't be enough because there would be no room for the tag.
        NOTE - enum has an extra size like 8 bytes for its tag which tells use which variant we have right now, but rust uses null pointer optimization instead of allocating 8 bytes tag 
        NOTE - null pointer optimization means a reference can never be null such as Option<&T> which is a pinter with 8 bytes length thus rust uses that reference or pointer as the tag with 8 bytes length for the current variant  
        NOTE - the size of the following enum is 28 (is equals to its largest variant size which belongs to the Text variant) + 8 (the tag size) bytes 

        pub enum UserID {
            Number(u64),
            Text(String),
        }

    ---------------------------------
*/
#[derive(BorshSerialize)] // NOTE - since UnorderedMap, LookupMap and UnorderedSet each one takes a vector of u8 as their key_prefix argument we have to bound the Storagekey enum to BorshSerialize trait to convert each variant into a vector of u8 using try_to_vec() method of the BorshSerialize trait 
// -> we've used an enum based storage key for better memory efficiency and avoiding data collision to keeps track of the persistent storage taken by the current collection (one of the following variant). 
// -> data collision could happen by UnorderedMap, LookupMap or UnorderedSet cause these hashmap based structure generate a hash from their keys. 
// -> in order not to have a duplicate key entry inside hashmap based structures we can use enum to avoid having some hash collision with two distinct keys.
// -> with enum we can be sure that there will be only one collection (one of the following variant) at a time inside the storage that has been pointed by the enum tag.
pub enum Storagekey{ 
    TokensPerOwner, 
    TokenPerOwnerInner{account_id_hash: CryptoHash}, //-- 32 bytes or 256 bits of the hash which will be 64 chars in hex
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner{token_type_hash: CryptoHash}, //-- 32 bytes or 256 bits of the hash which will be 64 chars in hex
    TokenTypesLocked,
}
