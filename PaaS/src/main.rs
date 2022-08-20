




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
use routerify::RouterService;
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
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //-- usize is the minimum size in os which is 32 bits
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let current_service = env::var("CURRENT_SERVICE").expect("⚠️ no current service variable set");
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_username = env::var("MONGODB_USERNAME").expect("⚠️ no db username variable set");
    let db_password = env::var("MONGODB_PASSWORD").expect("⚠️ no db password variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let auth_port = env::var("AYOUB_AUTH_PORT").expect("⚠️ no port variable set for auth service");
    let event_port = env::var("AYOUB_EVENT_PORT").expect("⚠️ no port variable set for event service");
    let game_port = env::var("AYOUB_GAME_PORT").expect("⚠️ no port variable set for game service");
    let nft_port = env::var("AYOUB_NFT_PORT").expect("⚠️ no port variable set for nft service");
    let auth_server_addr = format!("{}:{}", host, auth_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let event_server_addr = format!("{}:{}", host, event_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let game_server_addr = format!("{}:{}", host, game_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    let nft_server_addr = format!("{}:{}", host, nft_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    // let db_addr = format!("{}://{}:{}@{}:{}", db_engine, db_username, db_password, db_host, db_port); //------ UNCOMMENT THIS FOR PRODUCTION
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let (sender, receiver) = oneshot::channel::<u8>(); //-- oneshot channel for handling server signals - we can't clone the receiver of the oneshot channel














    // -------------------------------- cli args setup
    //
    // ------------------------------------------------------------------
    let username_cli = &String::new();
    let access_level_cli = &String::new();
    let args: Vec<String> = env::args().collect();
    let mut service_name = &args[1]; //-- since args[1] is of type String we must clone it or borrow its ownership using & to prevent args from moving, by assigning the first elem of args to service_name we'll lose the ownership of args (cause its ownership will be belonged to service_name) and args lifetime will be dropped from the ram 
    let service_port = &args[2];
    // if &args[1] == &"".to_string() && &args[2] == &"".to_string(){
    //     username_cli = &args[1]; //-- the username that we want to set his/her access level to dev
    //     access_level_cli = &args[1]; //-- the access level that must be used to update the user access_level
    // } else{
    //     username_cli = &args[3]; //-- the username that we want to set his/her access level to dev
    //     access_level_cli = &args[4]; //-- the access level that must be used to update the user access_level   
    // }
    
    
    
    
    
    
    
    
    
    
    
    
    
    // -------------------------------- service setup
    //
    // ------------------------------------------------------------
    let mut server_addr: Option<SocketAddr> = if service_name != &"".to_string() && service_port != &"".to_string(){ //-- if none of the argument was empty we set the server_addr to the one that we've got from the cli input otherwise we set it to None to fill it later using the current_service as the service_name 
        Some(format!("{}:{}", host, service_port).as_str().parse::<SocketAddr>().unwrap()) //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type
    } else{
        service_name = &current_service; //-- setting the serivce_name to the current_service read from the .env file cause it's empty read from the cli input
        None
    };

    









    

    
    
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
                // let mongodb_instance = init_db.GetMongoDbInstance().await; //-- the first argument of this method must be &self in order to have the init_db instance after calling this method, cause self as the first argument will move the instance after calling the related method and we don't have access to any field like init_db.url any more due to moved value error - we must always use & (like &self and &mut self) to borrotw the ownership instead of moving
                let mongodb_conn = &Client::with_uri_str(init_db.url.as_ref().unwrap()).await; //-- the first argument of this method must be &self in order to have the init_db instance after calling this method, cause self as the first argument will move the instance after calling the related method and we don't have access to any field like init_db.url any more due to moved value error - we must always use & (like &self and &mut self) to borrotw the ownership instead of moving
                let mongo_instance: Option<&Client> = Some(mongodb_conn.as_ref().unwrap()); 
                Some( //-- putting the Arc-ed db inside the Option
                    Arc::new( //-- cloning app_storage to move it between threads
                        ctx::app::Storage{ //-- defining db context 
                            id: Uuid::new_v4(),
                            db: Some(
                                ctx::app::Db{
                                    mode: init_db.mode,
                                    instance: Some(mongo_instance.unwrap()),
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
    let app_storage: Option<&'static Client> = match db.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => db.as_ref().unwrap().db.as_ref().unwrap().instance, //-- return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };










    // -------------------------------- set dev access level for passed in username in cli
    //
    // ---------------------------------------------------------------------------------------------
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







    

       
    
    
    
    

    // -------------------------------- building services, signal channel handling and server setup
    //
    // --------------------------------------------------------------------------------------------------------
    if service_name.as_str() == "auth"{
        info!("running auth server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let auth_router = routers::auth::register(app_storage.clone()).await;
        let auth_serivce = RouterService::new(auth_router).unwrap(); //-- making a new auth server by passing the generated storage
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut auth_server = Server::bind(&server_addr.unwrap()).serve(auth_serivce);
        // ------------------------------------------------
        // ------------------------------------------------
        let auth_graceful = auth_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = auth_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("auth server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        
        //
        // ...
        //

        Ok(())

    } else if service_name.as_str() == "event"{
        info!("running event server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let event_router = routers::event::register(app_storage.clone()).await;
        let event_serivce = RouterService::new(event_router).unwrap(); //-- making a new auth server by passing the generated storage
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut event_server = Server::bind(&server_addr.unwrap()).serve(event_serivce);
        // ------------------------------------------------
        // ------------------------------------------------
        let event_graceful = event_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = event_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("event server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature 

        // 
        // ...
        //

        Ok(())
    } else if service_name.as_str() == "game"{
        info!("running game server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let game_router = routers::game::register(app_storage.clone()).await;
        let game_serivce = RouterService::new(game_router).unwrap(); //-- making a new auth server by passing the generated storage
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut game_server = Server::bind(&server_addr.unwrap()).serve(game_serivce);
        // ------------------------------------------------
        // ------------------------------------------------
        let game_graceful = game_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = game_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("game server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature

        // 
        // ...
        //

        Ok(())
    } else{
        Ok(())
    }
    
    




}
