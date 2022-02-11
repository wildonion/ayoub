




/*
     -----------------------
    | Runtime struct fields
    |-----------------------
    |
    | -> id of the runtime
    | -> mode of the runtime
    | -> clients
    | -> current storage attched to runtime object
    | -> load balancer algorithm

*/




mod middlewares;
mod utils;
mod constants;
mod contexts;
mod schemas;
mod controllers;
mod routers;
use std::{net::SocketAddr, sync::{Arc, Mutex}};
use routers::auth;
use dotenv::dotenv;
use uuid::Uuid;
use std::env;
use log::{info, error};
use tokio::sync::oneshot::channel;
use hyper::{server::{Server, conn::AddrStream}, Response};
use hyper::service::{make_service_fn, service_fn};
use crate::contexts as ctx;















#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    
    




    // -------------------------------- environment variables setup
    //
    // ---------------------------------------------------------------------
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //-- usize is the minimum size in os which is 32 bits
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_username = env::var("DB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("DB_PASSWORD").expect("⚠️ no db password variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let port = env::var("AYOUB_PORT").expect("⚠️ no port variable set");
    let server_addr = format!("{}:{}", host, port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    // let db_addr = format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port);
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    
    
    
    
    


    
    // -------------------------------- app storage setup
    //
    // ---------------------------------------------------------------------
    let db = if db_engine.as_str() == "mongodb"{
        info!("switching to mongodb - {}", chrono::Local::now().naive_local());
        match ctx::app::Db::new().await{ //-- passing '_ as the lifetime of engine and url field which are string slices or pointers to a part of the String
            Ok(mut init_db) => {
                init_db.engine = Some(db_engine);
                init_db.url = Some(db_addr);
                info!("getting mongodb instance - {}", chrono::Local::now().naive_local());
                let mongodb_instance = init_db.GetMongoDbInstance().await; //-- the first argument of this method must be &self in order to have the init_db after calling this method cause self as the first argument will move the instance after calling the related method
                Some( //-- putting the Arc-ed db inside the Option
                    Arc::new( //-- cloning app_storage to move it between threads
                        ctx::app::Storage{ //-- defining db context 
                            id: Uuid::new_v4(),
                            db: Some(
                                ctx::app::Db{
                                    mode: init_db.mode,
                                    instance: Some(mongodb_instance),
                                    engine: init_db.engine,
                                    url: init_db.url,
                                }
                            ),
                        }
                    )
                )
            },
            Err(e) => {
                error!("init db error {} - {}", e, chrono::Local::now().naive_local());
                todo!()
            }
        }
    } else{
        todo!()
    };


    




    // -------------------------------- building runtime object
    //
    // NOTE - runtime object has a add_client() method in which a peer address will be pushed 
    //        into the clients vector thus its first argument must be defined as &mut self 
    //        and in order to push inside other threads we must put the runtime object inside a Mutex.
    // -------------------------------------------------------------------------------------------------------
    info!("initializing ayoub runtime - {}", chrono::Local::now().naive_local());
    let runtime = ctx::app::Runtime::new(db.clone()).await; //-- building a new runtime with specified db engine
    let arced_runtime = Arc::new(Mutex::new(runtime)); 





    



    // -------------------------------- making auth service
    //
    // ---------------------------------------------------------------------
    let auth_service = make_service_fn(move |conn: &AddrStream| {
        info!("making service function - {}", chrono::Local::now().naive_local());
        let addr = conn.remote_addr();
        let db = db.clone(); //-- db is not a variable in which its state changes during the runtime cause it's like a pointer which is pointing to the database actually thus we can clone it whenever we want
        let rt = arced_runtime.clone(); //-- clone is a deep copy so we don't have the old memory location (heap) of runtime object inside the rt variable 
        let service = service_fn(move |req| {
            info!("building auth service for client {} - {}", addr, chrono::Local::now().naive_local());
            let api = ctx::app::Api::new(Some(req), Some(Response::builder()), addr);
            rt.as_ref().lock().unwrap().add_client(addr); //-- as_ref() method will borrow the original value inside the wrapped type (Result or Option) 
            info!("bridging between current service and its controller - {}", chrono::Local::now().naive_local());
            auth::register(db.clone(), api) //-- registering app storage and the api on the auth router
        });
        async move { Ok::<_, constants::GenericError>(service) }
    });


    
    



    // -------------------------------- server and signal message setup
    //
    // -------------------------------------------------------------------------
    let server = Server::bind(&server_addr).serve(auth_service);
    let (sender, receiver) = channel::<u8>(); //-- oneshot channel for handling server signals - we can't clone the receiver of the mpsc channel 
    let graceful = server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));




    

    // -------------------------------- spawning server as an async task in the backgroun using tokio green threads
    //
    // -----------------------------------------------------------------------------------------------------------------------
    if let Err(e) = graceful.await{ //-- awaiting on the server to receive the shutdown signal
        sender.send(0); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty
        error!("server error {} - {}", e, chrono::Local::now().naive_local());
    }
    




    // TODO - if the number of clients reached too many requests shutdown the server
    // TODO - we can also distribute incoming requests from clients between multiple ayoub instances (load balancer)


    
    
    // TODO - add freez feature
    // TODO - detach db from the runtime object (set its storage field to None and db mode to off)
    // ...
    // sender.send(1);



    

    Ok(())


}
