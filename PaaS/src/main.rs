




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
mod services;
















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
        let auth_serivce = services::auth::AuthSvc::new(db.clone()).await; //-- making a new auth server by passing the generated storage
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut auth_server = Server::bind(&server_addr.unwrap()).serve(auth_serivce.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the auth_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //     BUILDING RUNTIME OBJECT FROM AUTH SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut auth_server as *mut Server<AddrIncoming, services::auth::AuthSvc>; //-- taking a mutable raw pointer to the auth_server to cast it to usize later
        let runtime = Some(
            ctx::rafael::env::Runtime::<services::auth::AuthSvc>{ //-- building runtime instance for the auth server 
                current_service: Some(auth_serivce), //-- later we can bind this service to a an address to run the its server - by this pattern service actors can communicate with each other before running their servers 
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(auth_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
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
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  

        // 
        // ...
        //

        Ok(())

    } else if service_name.as_str() == "event"{
        info!("running event server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let event_service = services::event::EventSvc::new(db.clone()).await;
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut event_server = Server::bind(&server_addr.unwrap()).serve(event_service.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the event_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //    BUILDING RUNTIME OBJECT FROM EVENT SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut event_server as *mut Server<AddrIncoming, services::event::EventSvc>; //-- taking a mutable raw pointer to the event_server to cast it to usize later
        let runtime = Some( //-- since the first param of the run() method of the Serverless trait defined as mutable the runtime object must be defined as mutable also
            ctx::rafael::env::Runtime::<services::event::EventSvc>{ //-- building runtime instance for the event server
                current_service: Some(event_service), //-- later we can bind this service to a an address to run the its server - by this pattern service actors can communicate with each other before running their servers 
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(event_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
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
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  

        // 
        // ...
        //

        Ok(())
    } else if service_name.as_str() == "game"{
        info!("running game server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let game_service = services::game::PlayerSvc::new(db.clone()).await;
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut game_server = Server::bind(&server_addr.unwrap()).serve(game_service.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the game_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //    BUILDING RUNTIME OBJECT FROM GAME SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut game_server as *mut Server<AddrIncoming, services::game::PlayerSvc>; //-- taking a mutable raw pointer to the game_server to cast it to usize later
        let runtime = Some( //-- since the first param of the run() method of the Serverless trait defined as mutable the runtime object must be defined as mutable also
            ctx::rafael::env::Runtime::<services::game::PlayerSvc>{ //-- building runtime instance for the game server
                current_service: Some(game_service), //-- later we can bind this service to a an address to run the its server - by this pattern service actors can communicate with each other before running their servers 
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(game_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
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
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  

        // 
        // ...
        //

        Ok(())
    } else if service_name.as_str() == "nft"{
        info!("running nft server on port {} - {}", service_port, chrono::Local::now().naive_local());
        let nft_service = services::nft::NftSvc::new(db.clone()).await;
        if server_addr == None{
            server_addr = Some(auth_server_addr);
        }
        let mut nft_server = Server::bind(&server_addr.unwrap()).serve(nft_service.clone()); //-- since Copy trait is not implemented we must clone the auth_service to prevent the type from moving - we have to define the nft_server as mutable cause we want to take a mutable raw pointer ot it
        // ------------------------------------------------
        //    BUILDING RUNTIME OBJECT FROM GAME SERVICE
        // ------------------------------------------------
        let mut raw_pointer_to_server = &mut nft_server as *mut Server<AddrIncoming, services::nft::NftSvc>; //-- taking a mutable raw pointer to the nft_server to cast it to usize later
        let runtime = Some( //-- since the first param of the run() method of the Serverless trait defined as mutable the runtime object must be defined as mutable also
            ctx::rafael::env::Runtime::<services::nft::NftSvc>{ //-- building runtime instance for the nft server
                current_service: Some(nft_service), //-- building runtime instance for the nft server
                link_to_server: Some(ctx::rafael::env::LinkToService(raw_pointer_to_server as usize)), //-- creating a link to the auth service by casting its mutable raw pointer to a usize which can be either 64 bits (8 bytes) or 32 bits (4 bytes) based on arch of the system
                id: Uuid::new_v4(),
                error: None,
                node_addr: Some(nft_server.local_addr()), //-- local address of this server which has been bound to
                last_crash: None,
                first_init: Some(Local::now().timestamp()),
            }
        );
        // ------------------------------------------------
        // ------------------------------------------------
        let game_graceful = nft_server.with_graceful_shutdown(ctx::app::shutdown_signal(receiver));
        if let Err(e) = game_graceful.await{ //-- awaiting on the server to receive the shutdown signal
            error!("game server error {} - {}", e, chrono::Local::now().naive_local());
        }
        // TODO - if the number of clients reached too many requests shutdown the server
        // TODO - call add_client() method to add an address into the clients vector
        // ...
        sender.send(0).unwrap(); //-- trigerring the shutdown signal on some bad event like DDOS or anything shitty 
        // sender.send(1); //-- freez feature
        // -----------------------------------------------------
        //   RUNNING SERVERLESS ENGINE OF THE RUNTIME INSTANCE 
        // -----------------------------------------------------
        let mut app = runtime.unwrap(); //-- to borrow the instance of the runtime as mutable we must define the app as mutable since the first param of the run() method is &mut self which is a mutable reference or pointer to all runtime instance fields
        app.run(); //-- run the runtime app in serverless mode  


        // 
        // ...
        // 


        Ok(())
    } else{
        Ok(())
    }
    
    




}
