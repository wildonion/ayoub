




/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  


                      
    Server Design Pattern Idea: https://github.com/hyperium/hyper/tree/master/examples
    Return Pointer from Functions Explanation: https://stackoverflow.com/a/57894943/12132470
    Return Pointer to a Structure Explanation: https://stackoverflow.com/questions/37789925/how-to-return-a-newly-created-struct-as-a-reference
    Rust Docs Gathered by wildonion: https://github.com/wildonion/extrust/tree/master/_docs



*/





// #![allow(unused)] //-- will let the unused vars be there - we have to put this on top of everything to affect the whole crate
#![macro_use] //-- apply the macro_use attribute to the root cause it's an inner attribute and will be effect on all things inside this crate 



use constants::MainResult;
use mongodb::Client;
use routerify::{RouterService, Router};
use std::{net::SocketAddr, sync::Arc, env};
use chrono::Local;
use dotenv::dotenv;
use uuid::Uuid;
use log::{info, error};
use tokio::sync::oneshot;
use hyper::server::{Server, conn::AddrIncoming};
use self::contexts as ctx; // use crate::contexts as ctx;
use ctx::rafael::env::Serverless; // NOTE - based on orphan rule Serverless trait is required to use the run() method on the runtime instance





mod middlewares;
mod utils;
mod constants;
mod contexts;
mod schemas;
mod controllers;
mod routers;
















#[tokio::main] //-- adding tokio proc macro attribute to make the main async
async fn main() -> MainResult<(), Box<dyn std::error::Error + Send + Sync + 'static>>{ //-- generic types can also be bounded to lifetimes ('static in this case) and traits inside the Box<dyn ... > - since the error that may be thrown has a dynamic size at runtime we've put all these traits inside the Box (a heap allocation pointer) and bound the error to Sync, Send and the static lifetime to be valid across the main function and sendable and implementable between threads
    
    



    



    


    // -------------------------------- environment variables setup
    //
    // ---------------------------------------------------------------------
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_username = env::var("MONGODB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("MONGODB_PASSWORD").expect("⚠️ no db password variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    // let db_addr = format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port); //------ UNCOMMENT THIS FOR PRODUCTION
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //-- usize is the minimum size in os which is 32 bits
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let port = env::var("AYOUB_PORT").expect("⚠️ no port variable set");
    let (sender, receiver) = oneshot::channel::<u8>(); //-- oneshot channel for handling server signals - we can't clone the receiver of the oneshot channel
    let server_addr = format!("{}:{}", host, port).as_str().parse::<SocketAddr>().unwrap();













    // -------------------------------- app storage setup
    //
    // ---------------------------------------------------------------------
    let empty_app_storage = Some( //-- putting the Arc-ed db inside the Option
        Arc::new( //-- cloning app_storage to move it between threads
            ctx::app::Storage{ //-- defining db context 
                id: Uuid::new_v4(),
                db: Some(
                    ctx::app::Db{
                        mode: ctx::app::Mode::Off,
                        instance: None,
                        engine: None,
                        url: None,
                    }
                ),
            }
        )
    );
    let db = if db_engine.as_str() == "mongodb"{
        info!("switching to mongodb - {}", chrono::Local::now().naive_local());
        match ctx::app::Db::new().await{
            Ok(mut init_db) => {
                init_db.engine = Some(db_engine);
                init_db.url = Some(db_addr);
                let mongodb_instance = init_db.GetMongoDbInstance().await; //-- the first argument of this method must be &self in order to have the init_db instance after calling this method, cause self as the first argument will move the instance after calling the related method and we don't have access to any field like init_db.url any more due to moved value error - we must always use & (like &self and &mut self) to borrotw the ownership instead of moving
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
                empty_app_storage //-- whatever the error is we have to return and empty app storage instance 
            }
        }
    } else{
        empty_app_storage
    };







    









    // -------------------------------- update to dev access level
    //
    // ------------------------------------------------------------------
    let args: Vec<String> = env::args().collect();
    let username_cli = &args[1];
    let access_level_cli = &args[2];
    if username_cli != &"".to_string() && access_level_cli != &"".to_string(){
        match utils::set_user_access(username_cli.to_owned(), access_level_cli.parse::<u8>().unwrap(), db.clone()).await{
            Ok(user_info) => {
                info!("access level for user {} has been updated successfully", username_cli);
                info!("updated user {:?}", user_info);
            },
            Err(empty_doc) => {
                info!("no user found for updating access level");
            },
        }
    } else{
        info!("no username has passed in to the cli; pass updating access level process");
    }







    

       
    
    
    
    

    // -------------------------------- building the ayoub server from the router
    //
    // --------------------------------------------------------------------------------------------------------
    let api = Router::builder()
        .scope("/auth", routers::auth::register().await)
        .scope("/event", routers::event::register().await)
        .scope("/game", routers::game::register().await)
        .build()
        .unwrap();

    info!("running auth server on port {} - {}", port, chrono::Local::now().naive_local());
    let ayoub_service = RouterService::new(api).unwrap();
    let ayoub_server = Server::bind(&server_addr).serve(ayoub_service);
    let ayoub_graceful = ayoub_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
    if let Err(e) = ayoub_graceful.await{ //-- awaiting on the server to receive the shutdown signal
        error!("auth server error {} - {}", e, chrono::Local::now().naive_local());
    }







        
        
        
        
        
        
        
        
        
        Ok(())
    





}
