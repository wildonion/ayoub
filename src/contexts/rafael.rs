



pub mod env{ 

    const APP_NAME: &str = "Ayoub";

    // TODO - env functions to mutate the state of the runtime object
    // TODO - try different IO streaming and future traits on a defined buffer from the following crates 
    // ...

    use std::sync::mpsc as std_mpsc;
    use futures::channel::mpsc as future_mpsc;
    use tokio::sync::mpsc as tokio_mpsc;
    use serde::{Serialize, Deserialize};
    use std::net::SocketAddr;
    use uuid::Uuid;
    use crate::services;
    use crate::contexts::app::Api;
    




    #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
    pub enum AppError{ //-- enum like union shares a common memory location between all its fields that means the space an enum needs is as much as the largest variant but unlike union the enum uses some extra memory to keep track of the enum variant which is called tag and is a pointer with 8 bytes length or 64 bits 
        OnRuntime, //-- caused by too much loading and requests
        OnStorage, //-- caused by storage services errors 
    }



    #[derive(Serialize, Deserialize)]
    pub struct LinkToService(pub usize); // NOTE - LinkToService contains a pointer to the current service address located inside the memory with usize as its size, u64 bits or 8 bytes or 32 btis or 4 bytes (based on arch)



    #[derive(Serialize, Deserialize)] // TODO - add wasm bindgen to compile this to wasm
    pub struct Runtime<S>{
        pub id: Uuid,
        pub current_service: S,
        pub link_to_server: LinkToService, //-- TODO - build the server type from usize of its pointer - due to the expensive cost of the String or str we've just saved a 64 bits or 8 bytes pointer (on 64 bits target) to the location address of the service inside the memory 
        pub error: Option<AppError>, //-- any runtime error cause either by the runtime itself or the storage crash
        pub node_addr: SocketAddr, //-- socket address of this node
        pub last_crash: Option<i64>, //-- last crash timestamp
        pub first_init: Option<i64>, //-- first initialization timestamp 
    }



    impl<S> Runtime<S>{ // TODO - add wasm bindgen attribute to compile this to wasm to call the wasm compiled methods using ayoub cli
        
        // Runtime serverless methods 
        // ...

    }



    pub trait Serverless{

        type Service; //-- the service type; game, auth, nft & etc...
        type App;

        fn run(&mut self); // NOTE - the type that this trait which must be implemented for must be defined as mutable

    }



    impl<S> Serverless for Runtime<S>{

        type Service = S;
        type App     = self::Api; 

        fn run(&mut self){ //-- the first param is a shared mutable pointer to the instance of the runtime 

        }


    }


    // impl Actor for Runtime{
        
    //     type Context = Context<Self>;

    //     fn started(&mut self, ctx: &mut Self::Context) {
            
    //     }

    // }




}