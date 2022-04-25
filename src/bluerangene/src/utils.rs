


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
    
    





    

use crate::*;  // load all defined crates, structs and functions from the root crate which is lib.rs in our case




/*
    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
    
        NOTE - enum has an extra size like 8 bytes, a 64 bits pointer which is big enough to store the current vairant address for its tag which tells use which variant we have right now, but rust uses null pointer optimization instead of allocating 8 bytes tag  
        NOTE - null pointer optimization means a reference can never be null such as Option<&T> which is a pinter with 8 bytes length thus rust uses that reference or pointer as the tag with 8 bytes length for the current variant  
        NOTE - none struct variants in Storagekey enum allocates zero byte for the current persistent storage once the tag point to their address at a time
        NOTE - the enum size with zero byte for each variants would be the largest size of its variant + 8 bytes tag which is 8 bytes in overall
        NOTE - an enum is the size of the maximum of its variants plus a discriminant value to know which variant it is, rounded up to be efficiently aligned, the alignment depends on the platform
        NOTE - an enum size is equals to a variant with largest size + 8 bytes tag
        NOTE - enum size with a single f64 type variant would be 8 bytes and with four f64 variants would be 16 bytes cause one 8 bytes (the tag) wouldn't be enough because there would be no room for the tag
        NOTE - the size of the following enum is 28 (is equals to its largest variant size which belongs to the Text variant) + 8 (the tag size) bytes 
        
        pub enum UserID {
            Number(u64),
            Text(String),
        }
        
    ---------------------------------------------------------------------------------------------------------------------------------------------------------------------
*/
#[derive(BorshSerialize)] // NOTE - since UnorderedMap, LookupMap and UnorderedSet each one takes a vector of u8 as their key_prefix argument we have to bound the Storagekey enum to BorshSerialize trait to convert each variant into a vector of u8 using try_to_vec() method of the BorshSerialize trait 
// -> we've used an enum based storage key for better memory efficiency and avoiding data collision to keeps track of the persistent storage taken by the current collection (one of the following variant). 
// -> data collision could happen by UnorderedMap, LookupMap or UnorderedSet cause these hashmap based structure generate a hash from their keys. 
// -> in order not to have a duplicate key entry inside hashmap based structures we can use enum to avoid having some hash collision with two distinct keys.
// -> with enum we can be sure that there will be only one collection (one of the following variant) at a time inside the storage that has been pointed by the enum tag.
pub enum Storagekey{ 
    TokensPerOwner, 
    TokenPerOwnerInner{account_id_hash: CryptoHash}, //-- 32 bytes or 256 bits of the hash which will be 64 chars in hex which is the account_id length
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner{token_type_hash: CryptoHash}, //-- 32 bytes or 256 bits of the hash which will be 64 chars in hex which is the account_id length
    TokenTypesLocked,
}











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



                                                             *** NEAR CONTRACTS IS BASED ON ACTOR DESIGN PATTERN ***


                    -----------------------------------------------------------------------------------------------------------------------------------------------
                    ----------- NEAR RUNTIME WILL CREATE ACTIONS RECEIPT FROM THE TRANSACTION EITHER FROM CONTRACT METHODS OR ONE OF THE FOLLOWING TYPE -----------
                    -----------------------------------------------------------------------------------------------------------------------------------------------
                        pub enum Action { 
                        CreateAccount(CreateAccountAction),
                        DeployContract(DeployContractAction),
                        FunctionCall(FunctionCallAction),
                        Transfer(TransferAction),
                        Stake(StakeAction),
                        AddKey(AddKeyAction),
                        DeleteKey(DeleteKeyAction),
                        DeleteAccount(DeleteAccountAction),
                    }



                                          -------------------------------------------------------------------------
                                        /                                                                          \
                                        ----------------------------------------------------------------------------
                                        Actor Definition : Actor is an object created from a struct that can sends 
                                                           future objects of messages or events or receipts to other 
                                                           actors asyncly their address through some message passing 
                                                           protocol like mpsc. 
                                        -----------------------------------------------------------------------------
                                        \                                                                           /
                                         ---------------------------------------------------------------------------
                        https://github.com/wildonion/coiniXerr/blob/d20d6e8e434987e354a6bc419a17a32dcb8ae432/src/utils/scheduler.rs#L44

                                         pub struct Account{
                                            pub id: CryptoHash // sha256 bits or 32 bytes or 64 char of hex of the hash of the account_id string which is the address of this actor  

                                        }


                                        impl Actor for Account{
                                            
                                            // impl some methods to transfer future objects of receipts or messages or events asyncly to other contract actors or receptor
                                            // ...

                                            fn start(){ // start a new thread pool for this actor

                                            }

                                            fn send(receptor) -> Future<Output=Receipts>{ // ge the result of the send() method using a .then() syntax

                                            }
                                        }




           Actor (address: alice.near)                                                                                                     Actor (address: bob.near)
              --------------------                                                                                                           --------------------                                                                                                  
            |                     |                                                                                                        |                     |
            |        Shard        |                                                                                                        |        Shard        |
            |   ---------------   |                                                                                                        |   ---------------   | 
            |  |               |  |  [Promise Message Passing (futures or receipts) Between Contract Actors Based on Pre-defined Actions]  |  |               |  |
            |  | Alice Account |  |         <---------- promise or future object contains data like funding balance ---------->            |  |  Bob Account  |  |
            |  |   ----------  |  |                                                                                                        |  |   ----------  |  |
            |  |  |contract A| |  |                       ---------------- [MPSC CHANNEL] ----------------                                 |  |  |contract B| |  |
            |  | / ----------  |  |                                                                                                        |  | / ----------  |  |
            |  / ---------------  |                                                                                                        |  / ---------------  |
             / -------------------                                                                                                         / -------------------
           /                                                                                                                             /
         /                                                                                                                             /
    contract-A.wasm                                                                                                               contract-B.wasm





0) receipts (or event inside the actor world!) are async messages which are in form of promise or future objects and will be created by runtime 
   from every transaction (action receipt) which contains either one of the above enum variant actions or a contract method to apply to a receiver (another contract actor)
   and can be scheduled to be ran later by passing them between actors (blocks or shards or contracts) asyncly (using the defined promise) through mpsc channel
   using the address of the second contract actor.
   
   

1) each contract belongs to a specific account and each account belongs to a specific shard which means 
   we can pass message between contracts or shards using actor design pattern (through the address of each actor) 
   and is more like every contract is an actor and every method of a contract is a transaction of different type 
   like payable ones and none payable ones which contains the sender and receiver account id and runtime 
   will create action receipt or message from these transactions (they can also mutate the state inside the contract). 



2) promises are future objects which contains some async message or receipt (data receipt) and can be scheduled to run and act on a given account_id based 
   on some action receipt (which will be created by runtime from a transaction or contract method like transfering fund to other contract or account) to run them inside 
   other contract actors (threads) by passing them asyncly through the mpsc channel using the address object of each contract actor to solve them (join or await on them) 
   inside other threads (contract actors) and get the result (awaited or joined response from the future or promise) of the promise after they get solved by passing the 
   callback method from the first contract actor into the .then() of the promise object and the callback contract has the data receipt of the first contract actor.


   
3) promise objects will be scheduled to sovle later inside other actors by passing them through mpsc channel asyncly 
   to other actors like creating a promise of refund action receipt to refund an account or other contract actors later.
    


4) we can await on multiple promises or future objects simultaneously in near contracts using promise_and; is more like joining on each of future object simultaneously.



5) since we can't have future objects in our contracts due to the fact that smart contract can't communicate with their outside 
   world and in order to solve the future we need tokio which is a socket based framework.



6) data receipt contains some data for the action receipt and data inside the action receipt is an Option 
   and if it was Some means we have awaited on that action and have some data.



7) action receipt contains vector of input data with their id for executing them based on the specified action and output data 
   vector which indicates data id and the receiver id or the other contract actor account.
   

   
8) for every incoming action receipt created by runtime from each transaction; runtime checks whether we have all the data receipt (data id inside the action receipt) 
   required for the execution if all the required data receipts are already in the storage, runtime can apply this action actor immediately otherwise we save this 
   receipt as a postponed action receipt and also we save pending data receipts count and a link from pending data receipts to the address of postponed action receipt; 
   now runtime will wait for all the missing data receipts to apply the postponed action receipt.  




*/



// ------------------------------ example of near actor design pattern
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// https://docs.near.org/docs/tutorials/contracts/xcc-rust
// -------------------------------------------------------------------------------------------------
// NOTE - the following function is a simulation of the near runtime which will create promise ActionReceipt from each transaction that can be either a pre defined action like funding or deleting account or a contract method
// NOTE - all methods of a contract must be called (from other contract actors) and changed the state of the contract using the following pattern which is an actor design pattern 
// NOTE - some pre defined methods are exist inside every account or contract actor and we can call them to create some promise ActionReceipts like transfer and delete account actions
// NOTE - all receipts or messages (Action or Data) must be in form of promise object in order to pass them between account or contract actors asyncly 
pub fn actor_ds_example(){
    
    


    // creating a new promise ActionReceipt from accountA.testnet account_id which will 
    // create a new empty promise ActionReceipt (async message) to pass it between 
    // contract actor through mpsc channel using actor address which is the 
    // hash of the account_id (accountA.testnet) in here
    let promise_id = env::promise_batch_create("accountA.testnet".to_string()); //-- a u64 bits or 8 bytes id which could be a pointer to the address of the promise
    
    


    env::promise_batch_action_function_call( //-- filling the created promise ActionReceipt with a transaction like calling the ft_balance_of() method of the current contract actor which is accountA.testnet account
        promise_id, //-- this is the id of the created promise which contains an empty promise ActionReceipt 
        b"ft_balance_of", //-- calling ft_balance_of() method of the current contract actor which is accountA.testnet
        &json!({"account_id": "accountB.testnet".to_string()}).to_string().into_bytes(), //-- data to be passed to the ft_balance_of() method in form of utf8 bytes
        0, //-- amount of yocto$NEAR to attach for this transaction which in our case is calling the ft_balance_of() method of the accountA.testnet contract actor
        5_000_000_000_000 //-- gas fee to attach
    );
    


    
    // the following is a callback promise ActionReceipt to receive the DataReceipt of the promise_id (the first promise) 
    // the ActionReceipt of this promise is dependent on the previous promise ActionReceipt whenever it gets solved we'll 
    // have the DataReceipt inside the following created promise ActionReceipt  
    let callback_promise_id = env::promise_batch_then( //-- creating the second promise which also will create an empty ActionReceipt to fulfill the callback promise with the incoming message or receipt which contains the data from the first promise ActionReceipt
        promise_id, //-- this is the id of the first promise ActionReceipt which contains the DataReceipt either pending, postponed or solved coming from the first promise ActionReceipt
        env::current_account_id(), //-- the current account_id (accountA.testnet) is the receiver of this created promise ActionReceipt    
    );
    



    // attacing a callback function to the callback promise ActionReceipt
    env::promise_batch_action_function_call(
        callback_promise_id, //-- this is the id of the second promise ActionReceipt which contains the DataReceipt from the first promise ActionReceipt
        b"my_callback", //-- the callback function which must be call after fulfilling the promise with the DataReceipt coming from the first promise ActionReceipt
        b"{}", //-- data to be passed to the my_callback() method in form of utf8 bytes
        0, //-- amount of yocto$NEAR to attach for this transaction which in our case is calling the ft_balance_of() method of the accountA.testnet contract actor
        5_000_000_000_000 //-- gas fee to attach
    );



    env::promise_return(callback_promise_id) //-- returning the solved DataReceipt of the callback promise 
    
    
}