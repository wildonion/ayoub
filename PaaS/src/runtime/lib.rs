





const APP_NAME: &str = "Rafael";
use wasm_bindgen::prelude::*;
use std::fmt;
use borsh::{BorshSerialize, BorshDeserialize};
use std::sync::mpsc as std_mpsc;
use serde::{Serialize, Deserialize};















// ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
// ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
//          RAFAEL WASM BINDING DATA STRUCTURES
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



#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone, Debug)] // TODO - use error derive proc macro attributes on the following enum fields
pub enum AppError{ //-- enum like union shares a common memory location between all its fields that means the space an enum needs is as much as the largest variant but unlike union the enum uses some extra memory to keep track of the enum variant which is called tag and is a pointer with 8 bytes length or 64 bits 
    OnRuntime, //-- caused by too much loading and requests
    OnStorage, //-- caused by storage services errors 
}



#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Service{
    Auth,
    Event,
    Game,
    Nft,
}



#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct LinkToService(pub usize); // NOTE - LinkToService contains a pointer to the current service address located inside the memory with usize as its size, u64 bits or 8 bytes or 32 btis or 4 bytes (based on arch)



#[derive(Clone, Debug)]
pub struct LoadBalancer; // TODO - clients -request-> middleware server -request-> main servers



// ---------------------------------------------------------
// wasm runtime structure and its methods to bind it into js  
// ---------------------------------------------------------

#[wasm_bindgen] //-- bounding the Runtime struct to wasm_bindgen proc macro attribute to compile it to wasm to generate a binding for js and convert it into js codes
#[derive(Serialize, Deserialize, Clone)]
pub struct RafaelRuntime{
    pub id: u8, //-- using u8 as the id instead of Uuid since the Uuid can't be convert to wasm
    pub current_service: self::Service, //-- current service variant
    pub link_to_server: Option<LinkToService>, //-- we've just saved a 64 bits or 8 bytes pointer (on 64 bits target) to the location address of the service inside the memory 
    pub error: Option<AppError>, //-- any runtime error caused either by the runtime itself or the storage crash
    #[wasm_bindgen(skip)] //-- skip exporting this field to wasm bindgen since the Copy trait is not implemented for String and heap data structures
    pub node_addr: String, //-- socket address of this node of type String since we can't use &str when we're bounding our struct into #[wasm_bindgen] proc macro attribute
    pub last_crash: Option<i64>, //-- last crash timestamp
    pub first_init: Option<i64>, //-- first initialization timestamp 
}


#[wasm_bindgen]
impl RafaelRuntime{
    


    // https://rustwasm.github.io/
    // https://docs.rs/wasm-bindgen-futures
    // https://crates.io/crates/wasm-react
    // https://crates.io/crates/wasm-bindgen
    // https://crates.io/crates/rayon
    // https://crates.io/crates/wasm-bindgen-rayon
    // split data using divide and conquer simd based design pattern and std::thread pool and mpsc
    // run multithreading in wasm to bind it into js to run in js using rayon
    // ...
    

    
    
    #[wasm_bindgen(constructor)]
    pub fn new(id: u8, current_service: &str, node_addr: &str) -> Self{
        let service = match current_service{
            "auth" => Service::Auth,
            "event" => Service::Event,
            "nft" => Service::Nft,
            _ => Service::Game,
        };

        Self{
            id, 
            current_service: service,
            link_to_server: None,
            error: None,
            node_addr: node_addr.to_string(),
            last_crash: None,
            first_init: Some(chrono::Local::now().timestamp()),
        }
    }


    #[wasm_bindgen(getter)]
    pub fn get_node_addr(&self) -> String{ //-- since we can't return reference with #[wasm_bindgen] thus we're returning the String instead of returning the &str 
        let node_addr = self.node_addr.clone(); //-- self.node_addr is a shared reference and we can't deref it since it doesn't implement the Copy trait thus we have to clone it to make a new node_addr variable and return that
        node_addr
    }



    // #[wasm_bindgen(getter)]
    // pub fn get_id(&self) -> String{ 
    //     let buffer_id = self.id.as_ref();
    //     let id_string = std::str::from_utf8(&buffer_id).unwrap().to_string();
    //     id_string
    // }


    #[wasm_bindgen]
    pub fn get_id(&self) -> u8{
        self.id
    } 



    #[wasm_bindgen(getter)]
    pub fn get_status(&self) -> String{
        let last_crash_time = self.last_crash.unwrap(); 
        let error_caused = self.error.unwrap();
        let status_message = format!("{} ::: The last crash was on {} caused by {:?}", APP_NAME, last_crash_time, error_caused);
        status_message
    }


    #[wasm_bindgen(getter)]
    pub fn get_current_service(&self) -> Service{
        self.current_service //-- no need to clone the current_service cause Copy trait is implemented for the Service enum
    }



}






