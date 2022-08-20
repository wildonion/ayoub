




use crate::{constants::*, schemas};
use futures::Future;
use mongodb::Client;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::Receiver;
use hyper::{Body, Server, server::conn::AddrIncoming};
use log::{info, error};














#[derive(Clone, Debug)] //-- can't bound Copy trait cause engine and url are String which are heap data structure 
pub struct Db{
    pub mode: Mode,
    pub engine: Option<String>,
    pub url: Option<String>,
    pub instance: Option<&'static Client>,
}

impl Default for Db{
    fn default() -> Db {
        Db{
            mode: self::Mode::Off,
            engine: None,
            url: None,
            instance: None,
        }
    }
}

impl Db{
    
    pub async fn new() -> Result<Db, Box<dyn std::error::Error>>{
        Ok(
            Db{ //-- building an instance with generic type C which is the type of the db client instance
                mode: super::app::Mode::On, //-- 1 means is on 
                engine: None, 
                url: None,
                instance: None,
            }
        )
    }
    
    pub async fn GetMongoDbInstance(&self) -> Client{ //-- it'll return an instance of the mongodb client - we set the first argument to &self in order to have the instance of the object later on after calling this method and prevent from moving
        Client::with_uri_str(self.url.as_ref().unwrap()).await.unwrap() //-- building mongodb client instance
    }

}






#[derive(Clone, Debug)]
pub struct Storage{
    pub id: Uuid,
    pub db: Option<Db>, //-- we could have no db at all
}



#[derive(Copy, Clone, Debug)]
pub enum Mode{ //-- enum uses 8 bytes (usize which is 64 bits on 64 bits arch) tag which is a pointer pointing to the current variant - the total size of this enum is 8 bytes tag + the largest variant size = 8 + 0 = 8 bytes; cause in our case On and Off variant both have 0 size
    On, //-- zero byte size
    Off, //-- zero byte size
}



#[derive(Clone, Debug)]
pub struct Cli{ // https://rust-cli.github.io/book/index.html
    pub service_name: String, // TODO - service_name argument
    pub port: u16, // TODO - port argument
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Response<'m, T>{
    pub data: Option<T>,
    pub message: &'m str,
    pub status: u16,
}



#[derive(Serialize, Deserialize)]
pub struct Nill<'n>(pub &'n [u8]); //-- this will be used for empty data inside the data field of the Response struct - 'n is the lifetime of the &[u8] type cause every pointer needs a lifetime in order not to point to an empty location inside the memory


pub async fn shutdown_signal(signal: Receiver<u8>){
    match signal.await{ //-- await on signal to get the message in down side of the channel
        Ok(s) => {
            if s == 0{
                info!("shutting down the server - {}", chrono::Local::now().naive_local());
                tokio::signal::ctrl_c().await.expect("failed to plugin CTRL+C signal to the server");
            } else if s == 1 { // TODO - freez the server
                // ...
            }
        },
        Err(e) => {
            error!("receiving error: [{}] cause sender is not available - {}", e, chrono::Local::now().naive_local())
        }
    }
}