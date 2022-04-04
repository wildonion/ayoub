








use futures::Future;
use mongodb::Client;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::Receiver;
use hyper::Body;
use log::{info, error};






unsafe impl Send for Api{}
unsafe impl Sync for Api {}


#[derive(Debug)]
pub struct Api{
    pub name: String,
    pub req: Option<hyper::Request<Body>>,
    pub res: Option<hyper::http::response::Builder>,
}


impl Api{

    // ---------------------------------------------------------------------------------------------------------------------
    // NOTE - we can borrow the req and res cause Request and Response structs are not bounded to Copy and Clone traits 
    //        thus cb closure (callback) arguments must be references to Request and Response objects.
    // NOTE - we can use as_ref() method to borrow the self.req and self.res cause as_ref() 
    //        converts Option<T> to Option<&T> then we can unwrap them to get the borrowed objects.
    // NOTE - don't put & behind self or borrow Api fields cause sharing Api fields between other threads 
    //        with & or borrowing the ownership is impossible caused by not implemented trait Clone (a super trait of Copy) 
    //        for hyper Request and Response structs error.
    // ---------------------------------------------------------------------------------------------------------------------

    pub fn new(request: Option<hyper::Request<Body>>, response: Option<hyper::http::response::Builder>) -> Self{
        Api{
            name: String::from(""),
            req: request,
            res: response,
        }
    }
    
    pub async fn post<F, C>(mut self, endpoint: &str, mut cb: F) -> Result<hyper::Response<Body>, hyper::Error> //-- defining self (an instance of the object) as mutable cause we want to assign the name of the api
                        where F: FnMut(hyper::Request<Body>, hyper::http::response::Builder) -> C, //-- capturing by mut T
                        C: Future<Output=Result<hyper::Response<Body>, hyper::Error>> + Send, //-- C is a future object which will be returned by the closure and has bounded to Send to move across threads
    {
        self.name = endpoint.to_string(); //-- setting the api name to the current endpoint
        let req = self.req.unwrap();
        let res = self.res.unwrap();
        let cb_res = cb(req, res).await.unwrap(); //-- this would be of type either hyper::Response<Body> or hyper::Error
        Ok(cb_res)
    }


    pub async fn get<F, C>(mut self, endpoint: &str, mut cb: F) -> Result<hyper::Response<Body>, hyper::Error> //-- defining self (an instance of the object) as mutable cause we want to assign the name of the api
                        where F: FnMut(hyper::Request<Body>, hyper::http::response::Builder) -> C, //-- capturing by mut T
                        C: Future<Output=Result<hyper::Response<Body>, hyper::Error>> + Send, //-- C is a future object which will be returned by the closure and has bounded to Send to move across threads
    {
        self.name = endpoint.to_string(); //-- setting the api name to the current endpoint
        let req = self.req.unwrap();
        let res = self.res.unwrap();
        let cb_res = cb(req, res).await.unwrap(); //-- this would be of type either hyper::Response<Body> or hyper::Error
        Ok(cb_res)
    }
}






#[derive(Clone, Debug)] //-- can't bound Copy trait cause engine and url are String which are heap data structure 
pub struct Db{
    pub mode: Mode,
    pub engine: Option<String>,
    pub url: Option<String>,
    pub instance: Option<Client>,
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
pub enum Mode{
    On,
    Off,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Response<'m, T>{
    pub data: Option<T>,
    pub message: &'m str,
    pub status: u32,
}



#[derive(Serialize, Deserialize)]
pub struct Nill<'n>(pub &'n [u8]); //-- this will be used for empty data inside the data field of the Response struct - 'n is the lifetime of the &[u8] type cause every pointer needs a lifetime in order not to point to an empty location inside the memory



pub async fn shutdown_signal(signal: Receiver<u8>){
    match signal.await{ //-- await on signal to get the message
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
