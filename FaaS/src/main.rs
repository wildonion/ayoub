








use rt::env;





fn main(){



    // TODO - calling between two wasm files using actors 
    //        every wasm file is an actor in which one of 
    //        its method can be called by another wasm file or actor
    // NOTE - wasi doesn't support threads and async methods runtimes like tokio  
    // ... 

    let rt = env::Runtime::<env::Service>{
        id: env::Uuid::new_v4(),
        current_service: Some(env::Service::Auth),
        link_to_server: None,
        error: None,
        node_addr: None,
        last_crash: None,
        first_init: None,
    };



    println!("Welcome to {:?} FaaS", env::APP_NAME);




}
