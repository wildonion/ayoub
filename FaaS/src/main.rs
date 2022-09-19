








use rt::env;





fn main(){




    // NOTE - wasi doesn't support async methods based runtimes like tokio  
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
