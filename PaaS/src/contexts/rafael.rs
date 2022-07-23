








//////////////////////////////////////////////////////////////////////////////////////
//////////////////// extending the runtime interface to have serverless trait methods
//////////////////////////////////////////////////////////////////////////////////////




pub mod env{ //-- rafael env functions to mutate the state of the runtime object like near-sdk env


    
    // TODO - use some kinda register setup process to get and mutate the vars of the env like near registers in its env module for promises or futures



    const APP_NAME: &str = "Rafless";
    use std::{fmt, env};
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
    //          RAFAEL DATA STRUCTURES & FUNCTIONS
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
    // ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag="event", content="data")] //-- the deserialized data of the following enum  will be : {"event": "runtime", "data": [{...RuntimeLog_instance...}, {...ServerlessLog_instance...}]} or {"event": "serverless", "data": [{...ServerlessLog_instance...}, {...ServerlessLog_instance...}]}
    #[serde(rename_all="snake_case")] //-- will convert all fields into snake_case
    pub enum EventVariant{
        Runime(Vec<RuntimeLog>),
        Serverless(Vec<ServerlessLog>),
    }



    #[derive(Serialize, Deserialize, Debug)]
    pub struct EventLog{ //-- an interface to capture the data about and event - this is the EVENT_JSON
        pub time: Option<i64>, //-- the time of the event data log
        #[serde(flatten)] //-- flatten to not have "event": {<EventVariant>} in the JSON, just have the contents of {<EventVariant>} which is the value of the data key itself - we can use #[serde(flatten)] attribute on a field of a struct or enum in those cases that we don't know about the number of exact fields inside the struct or enum or what's exactly inside the body of an api comming from the client to decode or map it into the struct or enum thus we can use this attribute to hold additional data that is not captured by any other fields of the struct or enum
        pub event: EventVariant, //-- the data which is a vector of all either Serverless or Runime variant events - we'll have {"time": 167836438974, "event": "event name, "data": [{...RuntimeLog_instance...}] or [{...ServerlessLog_instance...}]}
    }



    impl fmt::Display for EventLog{ //-- implementing the Display trait for the EventLog struct to show its instances' fields like EVENT_JSON:{"time": 167836438974, "event": "event name, "data": [{...RuntimeLog_instance...}] or [{...ServerlessLog_instance...}]} when we're calling logging functions like println!() which is a formatted stream of strings - any value or type that implements the Display trait can be passed to format_args!() macro, as can any Debug implementation be passed to a {:?} within the formatting string; Debug must be implemented for the type
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
            f.write_fmt( //-- writing some formatted information using format_args!() macro into the formatter instance which is `f`
                format_args!( //-- format_args!(), unlike its derived macros, avoids heap allocations
                    "EVENT_JSON:{}", //-- it'll start with EVENT_JSON:{} when you log the instance of the EventLog
                    &serde_json::to_string(self).map_err(|_| fmt::Error).unwrap() //-- formatting every field of the self which is the instance of the EventLog struct into the string to writ into the `f` and catch the fmt::error of each message or field if there was any when we're creating the stream by formatting the struct
                ) 
            ) // NOTE - we can print the string instance of the EventLog like so: println!("{:?}", event_log_instance.to_string()); since the Display trait is implemented for EventLog struct
        }
    }



    #[derive(Serialize, Deserialize, Clone, Debug)] // NOTE - Copy trait is not implemented for Box-ed types since the Box is a smart pointer to a heap allocated type and heap types have unknown size at compile time since they're not bounded to Sized trait
    pub struct RuntimeLog{ // TODO - initialize this inside the main() function
        pub id: u8,
        pub path: String, //-- the path of the log file in server with lifetime 'p
        #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
        pub requested_at: Option<i64>, //-- the time of the log request
        pub content: Box<[u8]>, //-- the array of utf8 bytes contains the content of the log inside the Box
    }



    #[derive(Serialize, Deserialize, Clone, Debug)] // NOTE - Copy trait is not implemented for Box-ed types since the Box is a smart pointer to a heap allocated type and heap types have unknown size at compile time since they're not bounded to Sized trait
    pub struct ServerlessLog{ // TODO - initialize this inside the main() function
        pub id: u8,
        pub path: String, //-- the path of the log file in server with lifetime 'p
        pub method: String, //-- the method name that the log data is captured for
        #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
        pub requested_at: Option<i64>, //-- the time of the log request
        pub content: Box<[u8]>, //-- the array of utf8 bytes contains the content of the log inside the Box
    }
    
    #[derive(Serialize, Deserialize, Copy, Clone)]
    pub struct LinkToService(pub usize); // NOTE - LinkToService contains a pointer to the current service address located inside the memory with usize as its size, u64 bits or 8 bytes or 32 btis or 4 bytes (based on arch)
    


    #[derive(Serialize, Deserialize, Copy, Clone, Debug)] // TODO - use error derive proc macro attributes on the following enum fields
    pub enum AppError{ //-- enum like union shares a common memory location between all its fields that means the space an enum needs is as much as the largest variant but unlike union the enum uses some extra memory to keep track of the enum variant which is called tag and is a pointer with 8 bytes length or 64 bits 
        OnRuntime, //-- caused by too much loading and requests
        OnStorage, //-- caused by storage services errors 
    }
    

    
    #[derive(Clone, Debug)]
    pub struct LoadBalancer; // TODO - clients -request-> middleware server -request-> main servers

    
    
    #[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
    pub enum FutureResult{
        Successful(Vec<u8>), //-- the successful result of the future object in form utf8 bytes
        Pending, //-- future is not ready
        Failed, //-- the fail result of the future object 
    }




    pub fn future_result(idx: u64) -> FutureResult{
        
        // TODO - 
        // ...
        // match super::env::future_get_result_of(idx){ // TODO - future_get_result_of() function must return Result<FutureResult, FutureError>
        //     Err(FutureResult::Pending) => FutureResult::Pending,
        //     Err(FutureResult::Failed) => FutureResult::Failed,
        //     Ok(()) => {
        //         let data = super::env::expect_register(read_register(ATOMIC_OP_REGISTER));
        //         FutureResult::Successful(data)
        //     } 
        // }
        
        todo!()
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
        fn refund(&mut self) -> Self; //-- &mut self is because we want to mutate the state if the runtime by refunding an account
        fn deposit(&mut self) -> Self; //-- &mut self is because we want to mutate the state if the runtime by adding some amount to an account 
        
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

        fn refund(&mut self) -> Self{

            todo!()

        }

        fn deposit(&mut self) -> Self{

            todo!()

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

            
            // TODO - use Arc<Mutex<T>> in multithreaded env and RefCell<Rc<T>> in single threaded env
            // TODO - actors will send encoded data through the mpsc channel from their free thread, so we have to deserialize them when we resolve them outside of the fulfilled future object 
            // TODO - every receipt is a transaction with a specific id which will be created by scheduling an ActionReceipt 
            // TODO - scheduling a promise of future object contains the method call (ActionReceipt) and get the resolved of the pending DataReceipt object from the executed future object inside a callback inside where we've scheduled the call
            // TODO - try different IO streaming and future traits on a defined buffer from the following crates like mpsc and Mutex data structures
            // TODO - consider every service a shard which can communicate (like executing each other's methods asyncly) with each other using their actors which has been implemented for each service through mpsc channels  
            // TODO - scheduling an event which is a future object contains an async message like calling one of the method of the second service 
            //        to be executed and triggered inside the second service and get the response inside a callback method using .then()
            // TODO - coming scheduled event from a thread of the first service actor inside a free thread of the second service actor 
            //        must be of type Arc<Mutex<T>> in order to avoid data races and dead locks 
            // TODO - sending async message from the current service to another serivce using actor that has been implemented for each service
            // TODO - vector of || async move{} of events for an event manager struct like event loop schema and call new event every 5 seconds from vector of event of closures 
            // TODO - use functional programming design pattern to call nested method on a return type of a struct method: events.iter().skip().take().map().collect()
            // ....  
            // ....
            // let resp = Schedule::on(service_address)
            //                  .data(arced_mutexed_data_object) //-- this is the data that must be executed on second service and it can be the name of a method inside that service 
            //                  .run_in_parallel()
            //                  .then(self.callback());
            // let resp = self.current_service.send(msg).to(another_serivce).await;

            
            todo!()
        }

        fn callback(&self) -> Self{
            

            // TODO - a callback method to get the response of the executed event in a specific service actor
            // ... 

            
            // -------------
            // if let syntax
            // -------------
            let fut_res = if let FutureResult::Successful(encoded_result) = super::env::future_result(0){ //-- getting the result of the future object only if it was successful
                // TODO - deserialize the result of the executed future object into a pre defined structure
                // ... 
            } else if let FutureResult::Failed = super::env::future_result(0){
                
            } else{

            };


            // -------------
            // match pattern
            // -------------
            match super::env::future_result(0){
                FutureResult::Successful(data) => {
                    
                },
                FutureResult::Failed => {

                },
                FutureResult::Pending => {

                },
                _ => {

                },
            }

            todo!()
        }


    }



}
