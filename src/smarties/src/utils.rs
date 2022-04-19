






use std::{sync::mpsc, thread, time::Instant}; // NOTE - mpsc means multiple thread can access the Arc<Mutex<T>> but only one of them can mutate the T
use near_sdk::env;




// NOTE - since can't compile socket in lib (wasm and bpf) mode, contracts can't interact with their outside worlds thus we can't have whether tokio or any web framework to handle async functions





// ------------------------------ message passing & multi threading (mpsc &  native threads)
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
pub fn simd<F>(number: u32, ops: F) -> Result<u32, String> where F: Fn(u8) -> u8 + std::marker::Send + 'static + Clone{ //-- in order to move the F between threads it must be bounded to Send trait
        
        
    let threads = 4; //-- the total number of all packs or chunks containing 8 bits which in this case is 4 cause our number is of type u32
    let (sender, receiver) = mpsc::channel::<u8>();
    let big_end_bytes = number.to_be_bytes(); //-- network bytes - since there are 4 chunks of 8 bits in the context of u32 bits there will be 4 chunks of 8 bits each chunk between 0 up to 255 
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