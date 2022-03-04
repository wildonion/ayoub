




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
mod apis;






use std::{net::SocketAddr, sync::Arc};
use dotenv::dotenv;
use uuid::Uuid;
use std::env;
use log::{info, error};
use crate::contexts as ctx;
use actix_web::{web, App, HttpServer, middleware::Logger};











// NOTE - Error, Send and Sync are traits which must be bounded to a type, since we don't know the type in compile time (will be specified at runtime) we must put these trait inside a Box with the dyn keword behind them cause we don't know how much size they will take space inside the memory  


#[actix_web::main]
async fn main() -> std::io::Result<()>{ 
    
    




    // -------------------------------- environment variables setup
    //
    // ---------------------------------------------------------------------
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    pretty_env_logger::init();
    dotenv().expect("⚠️ .env file not found");
    let io_buffer_size = env::var("IO_BUFFER_SIZE").expect("⚠️ no io buffer size variable set").parse::<u32>().unwrap() as usize; //-- usize is the minimum size in os which is 32 bits
    let environment = env::var("ENVIRONMENT").expect("⚠️ no environment variable set");
    let host = env::var("HOST").expect("⚠️ no host variable set");
    let fishuman_port = env::var("AYOUB_FISHUMAN_PORT").expect("⚠️ no port variable set for fishuman service");
    let fishuman_server_addr = format!("{}:{}", host, fishuman_port).as_str().parse::<SocketAddr>().unwrap(); //-- converting the host and port String into the as_str() then parse it based on SocketAddr generic type



    
    

    








    // -------------------------------- setup and run the server
    //
    // ---------------------------------------------------------------------
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/proposal")
                            .configure(apis::fishuman::register)
            )
            .wrap(Logger::default())
    })
    .bind(fishuman_server_addr).unwrap()
    .run()
    .await















}
