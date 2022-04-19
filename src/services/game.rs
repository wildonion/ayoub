



use crate::routers;
use crate::contexts as ctx;
use std::future::Future;
use hyper::{Body, Request, Response};
use hyper::service::Service; //-- an asynchronous function from a Request to a Response and is used to write middlewares and reusable network modules
use std::{pin::Pin, task::{Context, Poll}};
use uuid::Uuid;
use std::sync::Arc;
use std::marker::Send;
use std::net::SocketAddr;






 





#[derive(Clone, Debug)]
pub struct PlayerSvc{ 
    pub id: Uuid,
    pub clients: Vec<SocketAddr>,
    pub storage: Option<Arc<ctx::app::Storage>>, //-- we can have empty sotrage
}

impl PlayerSvc{ 
    
    pub async fn new(storage: Option<Arc<ctx::app::Storage>>, clients: Vec<SocketAddr>) -> PlayerSvc{
        PlayerSvc{
            id: Uuid::new_v4(),
            clients,
            storage,        
        }
    }

    pub fn add_client(&mut self, client: SocketAddr) -> Self{ // NOTE - runtime object has a add_client() method in which a peer address will be pushed into the clients vector thus its first argument must be defined as &mut self and in order to push inside other threads we must put the runtime object inside a Mutex.
        self.clients.push(client);
        PlayerSvc{
            id: self.id,
            clients: self.clients.clone(), //-- clients here is behind a mutable pointer (&mut self) and we must clone it cause trait Copy is not implemented for &mut self 
            storage: self.storage.clone(), //-- storage here is behind a mutable pointer (&mut self) and we must clone it cause trait Copy is not implemented for &mut self
        }
    }

    pub fn remove_client(&mut self, client_index: usize) -> Self{
        self.clients.remove(client_index);
        PlayerSvc{
            id: self.id,
            clients: self.clients.clone(),
            storage: self.storage.clone(),
        }
    }

}

impl<T> Service<T> for PlayerSvc{

    type Response = Svc; //-- the response type is an object of type Svc which handles registering routers
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
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
    
    
    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>>{ //-- Poll indicates whether a value is available or if the current task has been scheduled to receive a wakeup instead (Ready or Pending fields of the Poll enum)
        Poll::Ready(Ok(())) //-- calling Ready field of Poll enum means this future is ready for calling 
    }


    fn call(&mut self, req: Request<Body>) -> Self::Future{ //-- Body is the generic type of the Request struct
        let api = ctx::app::Api::new(Some(req), Some(Response::builder()));
        let res = routers::game::register(self.storage.clone(), api); //-- registering game routers for the game service - register method returns a result of hyper response based on calling one of the available routes
        Box::pin(async{ //-- returning the future response pinned to memory to solve later using .await - we can't mutate self.* in here cause the lifetime of the self must be static across a .await  
            res.await
        })
    }
}
