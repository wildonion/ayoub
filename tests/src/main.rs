






use std::{collections::HashMap, sync::{Arc, Mutex}, iter::Cloned};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fmt;







fn main(){
   




 
    

    let var_a = 32;
    let var_b = 535;
    let mut a = &var_a; //-- & is a pointer with a valid lifetime to the location of var_a type and it contains the address and the data of that type
    let mut b = &var_b; //-- & is a pointer with a valid lifetime to the location of var_b type and it contains the address and the data of that type
    ///// inline swapping : a, b = b, a and under the hood : a = &var_b, b = &var_a
    a = &var_b; //-- pointer of var_a must points to the location of var_b and after that it can have the data inside var_b 
    b = &var_a; //-- pointer of var_b must points to the location of var_a and after that it can have the data inside var_a


    




    

    /*

       ➔ in order to move the T between threads T must be bounded to Send and if it wasn't bound to Sync we have to clone it to move it between threads 
          and if it wasn't bound to Clone trait we must put it inside the Arc and to change it inside other thread we must put the Arc-ed type inside 
          the Mutex like mutating the Receiver<T> content inside other threads which must be in form Arc<Mutex<Receiver<T>>> since the Receiver is 
          not bounded to Sync (cause &Receiver is not bouned to Send cause there is no clone trait implemented for the Receiver thus it can't be copy 
          thus we can't have &Receiver and can't be clone) therefore we can't move it between threads due to the fact that the type T (Receiver in our case) 
          must be used by only one consumer or thread at a time.
       ➔ based on mpsc rules multiple threads can read the T and also T can be moved to those threads safely (since Send is implemented for that which 
          let us to have multiple owners for a type owned by other threads) but only single thread can write into the T to mutate it this is because 
          of rust ownership and borrowing rules which says that multiple immutable reference can be defined for T but one mutable reference can be there for T 
          and rust concurrency is based on this rule which is safe to use.
    
        -- Send means that a type is safe to move from one thread to another
        -- Sync makes the type safe (&T nmust be Send) to access shared reference across threads at the same time 
        -- Clone trait is implemented for the mpsc sender and is bounded to Send but not Sync and due to this reason we have to clone it in order we can have it in multiple threads (multi producer)
        -- Clone trait is not implemented for the mpsc receiver and we must put it inside Arc also is not Sync means it can't be referenced by multiple threads at the same time due to the fact that only one thread can mutate its content at a time (single consumer) thus we have to put it inside a Mutex
        -- in order to pass the receiver between threads safely and mutate its content by locking on it we must put the receiver inside Arc and Mutex like Arc<Mutex<Receiver<T>>>
    
    */
    let (s, r) = std::sync::mpsc::channel::<String>();
    let name = "wildonion".to_string();
    let mutexed_receiver = Mutex::new(r); //-- putting the receiver inside the Mutex to get its data by locking on it inside other threads
    let arced_mutexed_receiver = Arc::new(mutexed_receiver);
    s.send(name.clone()).unwrap(); //-- sending the mutated data to the channel


    for i in 0..3{
        let cloned_sender = s.clone();
        let cloned_name = name.clone();
        let cloned_arced_mutexed_receiver = arced_mutexed_receiver.clone();
        cloned_sender.send(name.clone()).unwrap(); //-- sending the name before get into the thread
        std::thread::spawn(move ||{ //-- we're sending data in this thread while we're receiving it at the same time
            while let Ok(mut data) = cloned_arced_mutexed_receiver.lock().unwrap().recv(){
                data = format!("new : {}-th-mutated-wildonion | old : {}", i, data); //-- mutating the data that we've just received
                println!("data >>> {}", data); //-- println!() macro uses the reference of the passed data thus we can have the data inside later scopes
                cloned_sender.send(data).unwrap(); //-- sending the mutated data to the channel
            }
        });
    } 





}
