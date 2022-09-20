








use rt::env;





fn main(){


    // NOTE - wasi doesn't support threads and async methods runtimes like tokio  
    // ... 

    // every runtime contains an specific service which can be compiled to the wasm 
    // every runtime has its own serverless methods which can be called by another wasm file or service

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