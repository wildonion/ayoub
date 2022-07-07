






/* 
                                                    ---------------------------------------------
                                                    THE IDEA BEHIND THE Api STRUCT DESIGN PATTERN
                                                    ---------------------------------------------

    based on the borrowing and ownership rules instead of garbage collection rule, the variable lifetime will be dropped after first usage 
    and to get this rule over we must either clone (Clone trait must be implemented for that type) the variable or borrow its ownership using 
    & in order to have it in later scopes therefore based on the desing pattern of the Api struct we can't bound it to Clone trait thus we can't clone 
    the api object or have api.clone() in multiple scopes inside a single crate like the nodejs express app object therefore the best design pattern 
    would be to return only one response object with a valid lifetime per request (cause the old one will be dropped after we're done with being inside the api methods) 
    in the whole app runtime based on matching the request on one of the available routes; since there is only one initialized api object which contains 
    the current request and a new response object inside the whole runtime in memory, there are no multiple of them at runtime 
    cause we're using match expression to detect the current route. 
    
    
    we've handled every incoming request using one the api object methods (post & get) in a 
    specific controller inside a single crate to implement the above idea!

    we can't borrow the api object cause it'll be a shared reference and can't dereference it 
    a shared reference across other threads and struct methods.
    
    shared reference can't dereference between threads and can't move out of it cause by moving or dereferencing it it'll lose its ownership and lifetime while some methods and 
    threads are using it; we can sovle this using as_ref() method wich converts a &wrapped type into &T or by cloning the type.

    I've choosed this pattern ➔ one api object which contains req and res object for the entire lifetime of the app since rust don't have garbage collection.



*/








use crate::routers;
use crate::contexts as ctx;
use std::future::Future;
use hyper::{Body, Request, Response};
use hyper::service::Service; //-- an asynchronous function from a Request to a Response and is used to write middlewares and reusable network modules
use std::{pin::Pin, task::{Context as TaskContext, Poll}};
use uuid::Uuid;
use std::sync::Arc;
use std::marker::Send;
use std::net::SocketAddr;
use actix::prelude::*;






 






/*
    
    ------------------------
    |     AUTH SERVICE 
    ------------------------
    |   Fields:
    |       id      -> Uuid
    |       clients -> Vector
    |       storage -> Storage option
    |
    |   Interfaces:
    |       Service
    |       Actor
    |
    |

*/
#[derive(Clone, Debug)]
pub struct AuthSvc{ 
    pub id: Uuid,
    pub clients: Vec<SocketAddr>,
    pub storage: Option<Arc<ctx::app::Storage>>, //-- we can have empty sotrage
}

impl AuthSvc{ 
    
    pub async fn new(storage: Option<Arc<ctx::app::Storage>>) -> AuthSvc{
        AuthSvc{
            id: Uuid::new_v4(),
            clients: vec![],
            storage,        
        }
    }

    pub fn add_client(&mut self, client: SocketAddr) -> Self{ // NOTE - runtime object has a add_client() method in which a peer address will be pushed into the clients vector thus its first argument must be defined as &mut self and in order to push inside other threads we must put the runtime object inside a Mutex.
        self.clients.push(client);
        AuthSvc{
            id: self.id,
            clients: self.clients.clone(), //-- clients here is behind a mutable pointer (&mut self) and we must clone it cause trait Copy is not implemented for &mut self 
            storage: self.storage.clone(), //-- storage here is behind a mutable pointer (&mut self) and we must clone it cause trait Copy is not implemented for &mut self
        }
    }

    pub fn remove_client(&mut self, client_index: usize) -> Self{
        self.clients.remove(client_index);
        AuthSvc{
            id: self.id,
            clients: self.clients.clone(),
            storage: self.storage.clone(),
        }
    }

}

impl<T> Service<T> for AuthSvc{

    type Response = Svc; //-- the response type is an object of type Svc which handles registering routers
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    

    fn poll_ready(&mut self, _: &mut TaskContext) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future{
        let id = self.id;
        let storage = self.storage.clone(); //-- cloning the storage to prevent its ownership from moving; Option implements the Clone trait so we can simply call the clone() method on self.storage  - as_deref() converts to Option<&Storage> and as_ref() converts wrapped T (by Arc or Option) to &T
        let clients = self.clients.clone(); //-- storage here is behind a mutable pointer (&mut self) and we must clone it cause trait Copy is not implemented for &mut self
        Box::pin(
            async move { 
                Ok(Self::Response { id, storage, clients }) //-- we can build the response object using Self::Response cause it's a alias for the Svc type
            }
        )
    }
}  










pub struct Svc{ //-- a struct to handle all incoming requests asynchronousely
    pub id: Uuid,
    pub clients: Vec<SocketAddr>,
    pub storage: Option<Arc<ctx::app::Storage>>,
}



impl Service<Request<Body>> for Svc{

    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>; //-- this is a is a wrapper around a kind of pointer which makes that pointer pin its value in place(stack or heap), preventing the value referenced by that pointer from being moved - we pinned the pointer of the Future object into memory cause we want to await on it later thus it shouldn't move from the memory by replacing with and pointing to a new value of a new variable
    
    
    fn poll_ready(&mut self, _: &mut TaskContext) -> Poll<Result<(), Self::Error>>{ //-- Poll indicates whether a value is available or if the current task has been scheduled to receive a wakeup instead (Ready or Pending fields of the Poll enum)
        Poll::Ready(Ok(())) //-- calling Ready field of Poll enum means this future is ready for calling 
    }


    fn call(&mut self, req: Request<Body>) -> Self::Future{ //-- Body is the generic type of the Request struct
        let api = ctx::app::Api::new(Some(req), Some(Response::builder())); //-- creating a new api object with a valid lifetime contains the incoming request and a new response
        let res = routers::auth::register(self.storage.clone(), api); //-- registering auth routers for the auth service - register method returns a result of hyper response based on calling one of the available routes
        Box::pin(async{ //-- returning the future response pinned to memory to solve later using .await - we can't mutate self.* in here cause the lifetime of the self must be static across a .await cause .await might be solved later thus the type that is used before and after it must have a valid lifetime which must be static 
            res.await
        })
    }
}












impl Actor for AuthSvc{

    type Context = Context<Self>; //-- building the Context object from the the Self which refers to the type that this actor is implementing for - Context object is a tool to control the lifecycle of an actor and is available only during the actor execution

    fn started(&mut self, ctx: &mut Self::Context) { 
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor is stopped");
    }
 

    // TODO - actor (threading, async msg, mpsc) borsh simd rpc based for cross serverless calls like schdeduling future objects for executing them inside other servers' functions
    // TODO - send borsh encoded async message between runtimes' server using actor mpsc channel and rpc for tenet idea
    // ...

}