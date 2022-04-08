




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



*/




mod middlewares;
mod utils;
mod constants;
mod contexts;
mod schemas;
mod controllers;
mod routers;
mod services;




use std::{net::SocketAddr, sync::Arc};
use dotenv::dotenv;
use uuid::Uuid;
use std::env;
use log::{info, error};
use tokio::sync::oneshot;
use hyper::server::Server;
// use crate::contexts as ctx;
use self::contexts as ctx;






// NOTE - to solve the `future is not `Send` as this value is used across an await` error we have to implement the Send trait for that type, since we don't know the type at compile time (it'll specify at runtime due to the logic of the code) we must put the trait inside the Box with the dyn keyword (object safe traits have unknown size at compile time) inside the return type of the function in second part of the Result 
// NOTE - Error, Send and Sync are object safe traits which must be bounded to a type, since we don't know the type in compile time (will be specified at runtime) we must put these trait inside a Box with the dyn keword behind them cause we don't know how much size they will take inside the memory






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
    let current_service = env::var("CURRENT_SERVICE").expect("⚠️ no current service variable set");
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_username = env::var("MONGODB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("MONGODB_PASSWORD").expect("⚠️ no db password variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let auth_port = env::var("AYOUB_AUTH_PORT").expect("⚠️ no port variable set for auth service");
    let event_port = env::var("AYOUB_EVENT_PORT").expect("⚠️ no port variable set for event service");
    let game_port = env::var("AYOUB_GAME_PORT").expect("⚠️ no port variable set for game service");
    let auth_server_addr = format!("{}:{}", host, auth_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let event_server_addr = format!("{}:{}", host, event_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let game_server_addr = format!("{}:{}", host, game_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    // let db_addr = format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port); //------ UNCOMMENT THIS FOR PRODUCTION
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let (sender, receiver) = oneshot::channel::<u8>(); //-- oneshot channel for handling server signals - we can't clone the receiver of the mpsc channel









    
    
    
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
        match ctx::app::Db::new().await{ //-- passing '_ as the lifetime of engine and url field which are string slices or pointers to a part of the String
            Ok(mut init_db) => {
                init_db.engine = Some(db_engine);
                init_db.url = Some(db_addr);
                info!("getting mongodb instance - {}", chrono::Local::now().naive_local());
                let mongodb_instance = init_db.GetMongoDbInstance().await; //-- the first argument of this method must be &self in order to have the init_db after calling this method cause self as the first argument will move the instance after calling the related method and we don't have access to init_db.url any more due to moved value error
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
                empty_app_storage
            }
        }
    } else{
        empty_app_storage
    };


    






     
    
    
    

    // -------------------------------- building services, signal channel handling and server setup
    //
    // --------------------------------------------------------------------------------------------------------
    if current_service.as_str() == "auth"{
        let auth_serivce = services::auth::AuthSvc::new(db.clone(), vec![]).await;
        let auth_server = Server::bind(&auth_server_addr).serve(auth_serivce);
        let auth_graceful = auth_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = auth_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("auth server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        Ok(())
    } else if current_service.as_str() == "event"{
        let event_service = services::event::EventSvc::new(db.clone(), vec![]).await;
        let event_server = Server::bind(&event_server_addr).serve(event_service);
        let event_graceful = event_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = event_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("event server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        Ok(())
    } else if current_service.as_str() == "game"{
        let game_service = services::game::PlayerSvc::new(db.clone(), vec![]).await;
        let game_server = Server::bind(&game_server_addr).serve(game_service);
        let game_graceful = game_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = game_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("game server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        Ok(())
    } else {
        Ok(())
    }
    
    








}
