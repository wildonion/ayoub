




use crate::{constants::*, schemas};
use futures::Future;
use mongodb::Client;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::Receiver;
use hyper::{Body, Server, server::conn::AddrIncoming};
use log::{info, error};












type Callback = Box<dyn 'static + FnMut(hyper::Request<Body>, hyper::http::response::Builder) -> CallbackResponse>; //-- capturing by mut T - the closure inside the Box is valid as long as the Callback is valid due to the 'static lifetime and will never become invalid until the variable that has the Callback type drop
type CallbackResponse = Box<dyn Future<Output=GenericResult<hyper::Response<Body>, hyper::Error>> + Send + 'static>; //-- CallbackResponse is a future object which will be returned by the closure and has bounded to Send to move across threads - the future inside the Box is valid as long as the CallbackResponse is valid due to the 'static lifetime and will never become invalid until the variable that has the CallbackResponse type drop

unsafe impl Send for Api{}
unsafe impl Sync for Api {}



pub struct Api{
    pub name: String,
    pub req: Option<hyper::Request<Body>>,
    pub res: Option<hyper::http::response::Builder>,
    pub callback: Option<Callback>, //-- the generic type of the callback field is the Callback type which is FnMut and a Future object for its return type inside the Box
    pub access_level: Option<u8>, //-- it might be None and the api doesn't require an access level
}



impl Api{

    // -----------------------------------------------------------------------------------------------------------------------------
    // NOTE - we can borrow the req and res cause Request and Response structs are not bounded to Copy and Clone traits 
    //        thus cb closure (callback) arguments must be references to Request and Response objects.
    // NOTE - we can use as_ref() method to borrow the self.req and self.res cause as_ref() 
    //        converts Option<T> to Option<&T> then we can unwrap them to get the borrowed objects.
    // NOTE - don't put & behind self or borrow Api fields cause sharing Api fields between other threads using a shared reference
    //        with & or borrowing the ownership is impossible cause by not implemented trait Clone (a super trait of Copy) 
    //        for hyper Request and Response structs error.
    // NOTE - the body of the `cb` in post and get methods is an async move{} means it'll return a future object
    //        which we can solve it using .await later.
    // NOTE - since we can't put & behind the mut self thus we can't have the instance of the Api in later scopes
    //        after calling its post or get methods and due to this fact we've built controllers which implements
    //        only one Api instance per writing api pattern, means since we can have only one Api instance inside
    //        a crate therefore we must have one controller per each Api instance to handle the incoming request
    //        inside that controller which is related to a specific route (MVC like design pattern).
    // NOTE - we can't have api.post().await and api.get().await inside the same scope from one instance since with the first 
    //        use the api instance will be moved and its lifetime will be dropped due to the above third NOE.  
    // NOTE - since both api.post() and api.get() methods are async thus we have to await on them to run their callback closures
    //        which contain the logic of the whole controller. 
    // -----------------------------------------------------------------------------------------------------------------------------

    pub fn new(request: Option<hyper::Request<Body>>, response: Option<hyper::http::response::Builder>) -> Self{
        Api{
            name: String::from(""),
            req: request,
            res: response,
            callback: None, // TODO - caching using closures: https://github.com/wildonion/extrust/blob/4a3e72184ea5159d0ec6d4e8325e481019023b4f/_trash/_garbage.rs#L11
            access_level: None, // TODO
        }
    } 
    
    pub async fn post<F, C>(mut self, endpoint: &str, mut cb: F) -> GenericResult<hyper::Response<Body>, hyper::Error> //-- defining self (an instance of the object) as mutable cause we want to assign the name of the api; since we didn't borrow the self (the instance itself) using & we can't call this method for the second call cause the ownership of the instance will be moved in first call  
                        where F: FnMut(hyper::Request<Body>, hyper::http::response::Builder) -> C, //-- capturing by mut T
                        C: Future<Output=GenericResult<hyper::Response<Body>, hyper::Error>> + Send, //-- C is a future object which will be returned by the closure and has bounded to Send to move across threads
    {
        self.name = endpoint.to_string(); //-- setting the api name to the current endpoint
        let req = self.req.unwrap();
        let res = self.res.unwrap();
        let cb_res = cb(req, res).await.unwrap(); //-- calling the passed in closure to the post method by passing the request and response objects since this closure callback contains the body of the controller method - this would be of type either hyper::Response<Body> or hyper::Error
        Ok(cb_res)
    }


    pub async fn get<F, C>(mut self, endpoint: &str, mut cb: F) -> GenericResult<hyper::Response<Body>, hyper::Error> //-- defining self (an instance of the object) as mutable cause we want to assign the name of the api; since we didn't borrow the self (the instance itself) using & we can't call this method for the second call cause the ownership of the instance will be moved in first call  
                        where F: FnMut(hyper::Request<Body>, hyper::http::response::Builder) -> C, //-- capturing by mut T
                        C: Future<Output=GenericResult<hyper::Response<Body>, hyper::Error>> + Send, //-- C is a future object which will be returned by the closure and has bounded to Send to move across threads
    {
        self.name = endpoint.to_string(); //-- setting the api name to the current endpoint
        let req = self.req.unwrap();
        let res = self.res.unwrap();
        let cb_res = cb(req, res).await.unwrap(); //-- calling the passed in closure to the post method by passing the request and response objects since this closure callback contains the body of the controller method - this would be of type either hyper::Response<Body> or hyper::Error
        Ok(cb_res)
    }

    pub async fn set_name(&mut self, endpoint: &str){ //-- we must define self as mutable cause we want to change the name field
        let endpoint_name = endpoint.to_string();
        self.name = endpoint_name;
    }

    pub async fn get_name(&self) -> String{
        let endpoint_name = self.name.to_string(); //-- self.name is the dereferenced value of the &self.name and will be done automatically by the compiler 
        endpoint_name 
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