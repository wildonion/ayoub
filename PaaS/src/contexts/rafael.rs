








//////////////////////////////////////////////////////////////////////////////////////
//////////////////// extending the runtime interface to have serverless trait methods
//////////////////////////////////////////////////////////////////////////////////////




pub mod env{ //-- env functions to mutate the state of the runtime object


    const APP_NAME: &str = "Rafless";
     

    
    // TODO - try different IO streaming and future traits on a defined buffer from the following crates like mpsc and Mutex data structures



    use crate::services;
    use crate::contexts::app::Api;
    use futures::channel::mpsc as future_mpsc;
    use tokio::sync::mpsc as tokio_mpsc;
    use futures::join as futures_join;
    use futures_util::join as futures_util_join;
    use tokio::join as tokio_join;
    use borsh::{BorshSerialize, BorshDeserialize};
    use uuid::Uuid;
    use std::net::SocketAddr;
    use serde::{Serialize, Deserialize};
    use rayon::join as rayon_join;













    
    


    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    //                RAFAEL DATA STRUCTURES
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    
    #[derive(Serialize, Deserialize, Copy, Clone)]
    pub struct LinkToService(pub usize); // NOTE - LinkToService contains a pointer to the current service address located inside the memory with usize as its size, u64 bits or 8 bytes or 32 btis or 4 bytes (based on arch)
    


    #[derive(Serialize, Deserialize, Copy, Clone, Debug)] // TODO - use error derive proc macro attributes on the following enum fields
    pub enum AppError{ //-- enum like union shares a common memory location between all its fields that means the space an enum needs is as much as the largest variant but unlike union the enum uses some extra memory to keep track of the enum variant which is called tag and is a pointer with 8 bytes length or 64 bits 
        OnRuntime, //-- caused by too much loading and requests
        OnStorage, //-- caused by storage services errors 
    }
    
    
    
    // NOTE - #[wasm_bindgen] proc macro attribute is used to compile structs and their methods into .wasm file to bind it into js to run in browser
    // NOTE - can't bind the #[wasm_bindgen] proc macro attribute since it doesn't supprt generic type, lifetimes (means can't return a borrowed type inside the structure method) 
    //        and raw unix socket since we can't run socket server inside the browser or js; we can only setup websocket client. 
    // NOTE - struct public fields are automatically infered to generate accessors in js but they're required to be Copy thus we have to implement the
    //        Copy trait for our structs and enums; since the Copy is not implemented for heap data structures due to their unknown size at compile time
    //        we must avoid exporting those fields into wasm using #[wasm_bindgen(skip)] and use setter and getter methods in struct impl block. 
    #[derive(Serialize, Deserialize)]
    pub struct Runtime<S>{ 
        pub id: Uuid,
        pub current_service: Option<S>,
        pub link_to_server: Option<LinkToService>, //-- we've just saved a 64 bits or 8 bytes pointer (on 64 bits target) to the location address of the service inside the memory 
        pub error: Option<AppError>, //-- any runtime error caused either by the runtime itself or the storage crash
        pub node_addr: Option<SocketAddr>, //-- socket address of this node
        pub last_crash: Option<i64>, //-- last crash timestamp
        pub first_init: Option<i64>, //-- first initialization timestamp 
    }
    





    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    //              RAFAEL SERVERLESS METHODS
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    pub trait Serverless{ /////// a functional Serverless trait for Runtimes

        type Service; //-- the service type; game, auth, nft & etc...
        type App;
        type Cost; //-- storage cost of api calls or the total cost of the serverless trait method calls during an especific period of time based on amount of CPU, network, and IO, and the amount of data already stored in runtime storage which is the VPS ram 


        ////////////////////////////////////////////////////////////////////////////////
        ///// FOLLOWING METHODS MIGHT BE CALLED MORE THAN 1000 TIMES PER SECOND BY USERS 
        ///// THUS WE HAVE CODE THEM AS EFFICIENT AS POSSIBLE.
        ////////////////////////////////////////////////////////////////////////////////


        fn run(&mut self) -> Self; // NOTE - the type that this trait which must be implemented for must be defined as mutable - the return type is the type that this trait will be implemented for
        fn stop() -> Self; // NOTE - this is not object safe trait since we're returning the Self 
        fn schedule(&self) -> Self; //-- NOTE - not an object safe trait since we have self in method param and returning signature 
        fn callback(&self) -> Self;
        
    }



    impl<S> Serverless for Runtime<S>{

        type Service = S;
        type App     = self::Api; 
        type Cost    = u128; 

        fn run(&mut self) -> Self{ //-- the first param is a shared mutable pointer to the instance of the runtime 
            Self{
                id: Uuid::new_v4(),
                current_service: None,
                link_to_server: None,
                error: None,
                node_addr: None,
                last_crash: None,
                first_init: Some(chrono::Local::now().timestamp()),
            }
        }

        fn stop() -> Self{
            Self{
                id: Uuid::new_v4(),
                current_service: None,
                link_to_server: None,
                error: None,
                node_addr: None,
                last_crash: None,
                first_init: Some(chrono::Local::now().timestamp()),
            }
        }

        fn schedule(&self) -> Self{

            
            // TODO - consider every service a shard which can communicate (like executing each other's methods asyncly) with each other using their actors which has been implemented for each service through mpsc channels  
            // TODO - scheduling an event which is a future object contains an async message like calling one of the method of the second service 
            //        to be executed and triggered inside the second service and get the response inside a callback method using .then()
            // TODO - coming scheduled event from a thread of the first service actor inside a free thread of the second service actor 
            //        must be of type Arc<Mutex<T>> in order to avoid data races and dead locks 
            // TODO - sending async message from the current service to another serivce using actor that has been implemented for each service
            // TODO - vector of || async move{} of events for an event manager struct 
            // TODO - call new event every 5 seconds from vector of event of closures 
            // ....  
            // ....
            // let resp = Schedule::on(service_address)
            //                  .data(arced_mutexed_data_object) //-- this is the data that must be executed on second service
            //                  .run_in_parallel()
            //                  .then(self.callback());
            // let resp = self.current_service.send(msg).to(another_serivce).await;
            
            
            todo!()
        }

        fn callback(&self) -> Self{
            
            // TODO - a callback method to get the response of the executed event in a specific service actor
            // ... 
            todo!()
        }


    }



}
