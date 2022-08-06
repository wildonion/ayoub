






use std::{collections::HashMap, sync::{Arc, Mutex}, iter::Cloned};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fmt;







fn main(){
   


    
    let (s, r) = std::sync::mpsc::channel::<String>();
    let name = "wildonion".to_string();
    let mutexed_receiver = Mutex::new(r); //-- putting the receiver inside the Mutex to get its data by locking on it inside other threads since the Sync is not implemented for the receiver and in order to get its data inside other threads we have to make clonable using Arc and some kina syncable using Mutext
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
