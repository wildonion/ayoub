





mod constants;
use wasm_bindgen::prelude::*;
use borsh::{BorshSerialize, BorshDeserialize};
use std::sync::mpsc as std_mpsc;
use serde::{Serialize, Deserialize};



















// ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
// ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
//          RAFAEL WASM BINDING DATA STRUCTURES
// ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡
// ‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡‡

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



// ---------------------------------------------------------
// wasm runtime structure and its methods to bind it into js  
// ---------------------------------------------------------
// NOTE - #[wasm_bindgen] proc macro attribute is used to compile structs and their methods into .wasm file to bind it into js to run in browser
// NOTE - can't bind the #[wasm_bindgen] proc macro attribute since it doesn't supprt generic type, lifetimes (means can't return a borrowed type inside the structure method) 
//        and raw unix socket since we can't run socket server inside the browser or js; we can only setup websocket client. 
// NOTE - struct public fields are automatically infered to generate accessors in js but they're required to be Copy thus we have to implement the
//        Copy trait for our structs and enums; since the Copy is not implemented for heap data structures due to their unknown size at compile time
//        we must avoid exporting those fields into wasm using #[wasm_bindgen(skip)] and use setter and getter methods in struct impl block. 
#[wasm_bindgen] //-- bounding the Runtime struct to wasm_bindgen proc macro attribute to compile it to wasm to generate a binding for js and convert it into js codes
#[derive(Serialize, Deserialize, Clone)]
pub struct RafaelRuntime{
    pub id: u8, //-- using u8 as the id instead of Uuid since the Uuid can't be convert to wasm
    pub current_service: self::Service, //-- current service variant
    pub link_to_server: Option<LinkToService>, //-- we've just saved a 64 bits or 8 bytes pointer (on 64 bits target) to the location address of the service inside the memory 
    pub error: Option<AppError>, //-- any runtime error caused either by the runtime itself or the storage crash
    #[wasm_bindgen(skip)] //-- skip exporting this field to wasm bindgen since the Copy trait is not implemented for String and heap data structures thus we can't have String field in a structure which has bounded to #[wasm_bindgen] proc macro attribute or other heap data structures
    pub node_addr: String, //-- socket address of this node of type String since we can't use &str when we're bounding our struct into #[wasm_bindgen] proc macro attribute
    pub last_crash: Option<i64>, //-- last crash timestamp
    pub first_init: Option<i64>, //-- first initialization timestamp 
}


#[wasm_bindgen]
impl RafaelRuntime{
    


    // https://sycamore-rs.netlify.app/
    // https://github.com/seed-rs/seed
    // https://github.com/yewstack/yew
    // https://rustwasm.github.io/
    // https://docs.rs/wasm-bindgen-futures
    // https://crates.io/crates/wasm-react
    // https://crates.io/crates/wasm-bindgen
    // https://crates.io/crates/rayon
    // https://crates.io/crates/wasm-bindgen-rayon
    // TODO - use yew to build web pages using its html!{} macro in rust macro which will be compiled to wasm and bundled with rollup  
    // TODO - split data using divide and conquer simd based design pattern and std::thread pool and mpsc
    // TODO - run multithreading in wasm to bind it into js to run in js using rayon
    // TODO - use functional programming design pattern to call nested method on a return type of a struct method
    // ...
    

    
    
    // JS CODE => let rt = new RafaelRuntime(1837, "auth", "0.0.0.0:2424"); 
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



    #[wasm_bindgen]
    pub fn get_id(&self) -> u8{
        self.id
    } 



    #[wasm_bindgen(getter)]
    pub fn get_status(&self) -> String{
        let last_crash_time = self.last_crash.unwrap(); 
        let error_caused = self.error.unwrap();
        let status_message = format!("{} ::: The last crash was on {} caused by {:?}", constants::APP_NAME, last_crash_time, error_caused);
        status_message
    }



    #[wasm_bindgen(getter)]
    pub fn get_current_service(&self) -> Service{
        self.current_service //-- no need to clone the current_service cause Copy trait is implemented for the Service enum
    }



    #[wasm_bindgen(setter)]
    pub fn set_service(&mut self, current_service: &str) -> Self{ //-- since we want to mutate tbe state of structure we've defined the first param as &mut self
        let service = match current_service{
            "auth" => Service::Auth,
            "game" => Service::Game,
            "event" => Service::Event,
            _ => Service::Nft,
        };
        Self{ 
            id: self.id,
            current_service: service, //-- new field
            link_to_server: self.link_to_server,
            error: self.error,
            node_addr: self.node_addr.clone(), //-- cloning the self.node_addr since cannot move out of `self.node_addr` which is behind a mutable reference move occurs because `self.node_addr` has type `std::string::String`, which does not implement the `Copy` trait
            last_crash: self.last_crash,
            first_init: self.first_init, 
        }
    }


}






