


use crate::constants::{ACCESS_GRANTED, NOTFOUND_ROUTE};
use crate::contexts as ctx;
use std::future::Future;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use hyper::{Method, header, StatusCode, Body, Request, Response};
use hyper::service::Service; //-- an asynchronous function from a Request to a Response and is used to write middlewares
use std::{pin::Pin, task::{Context, Poll}};
use ctx::app::Nill;
use log::{info, warn, error};
use uuid::Uuid;
use std::sync::Arc;
use std::marker::{Send, Sync};




 



pub struct AuthSvc<C: Sync + Send + 'static>{ //-- C is the generic type for the db client instance and must be sync, send (shareable between threads) and have a valid lifetime across .await (static)
    pub id: Uuid,
    pub storage: Option<Arc<ctx::app::Storage<C>>>, //-- we can have no storage at all
}

impl<T, C: Sync + Send + 'static> Service<T> for AuthSvc<C>{ //-- C is the generic type for the db client instance and must be sync, send (shareable between threads) and have a valid lifetime across .await (static)

    type Response = Svc<C>; //-- C is the generic type for the db client instance in Svc struct
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Sync + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future{
        let id = self.id;
        let storage = self.storage.clone(); //-- cloning the storage to prevent its ownership from moving; Option implements the Clone trait so we can simply call the clone() method on self.storage  - as_deref() converts to Option<&Storage> and as_ref() converts wrapped T (by Arc or Option) to &T
        Box::pin(
            async move { 
                Ok(Svc { id, storage })
            }
        )
    }
}  








pub struct Svc<C: Sync + Send + 'static>{ //-- C is the generic type for the db client instance and must be sync, send (shareable between threads) and have a valid lifetime across .await (static)
    pub id: Uuid,
    pub storage: Option<Arc<ctx::app::Storage<C>>>, //-- we can have no storage at all
}



impl<C: Sync + Send + 'static> Service<Request<Body>> for Svc<C>{ //-- C is the generic type for the db client instance and must be sync, send (shareable between threads) and have a valid lifetime across .await (static)

    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>> + Send>>; //-- this is a is a wrapper around a kind of pointer which makes that pointer pin its value in place(stack or heap), preventing the value referenced by that pointer from being moved - we pinned the pointer of the Future object into memory cause we want to await on it later thus it shouldn't move from the memory by replacing with and pointing to a new value of a new variable
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Sync + Send + 'static>>;
    
    
    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>>{ //-- Poll indicates whether a value is available or if the current task has been scheduled to receive a wakeup instead (Ready or Pending fields of the Poll enum)
        Poll::Ready(Ok(())) //-- calling Ready field of Poll enum means this future is ready for calling 
    }



    fn call(&mut self, req: Request<Body>) -> Self::Future{ //-- Body is the generic type of the Request struct
        
        let res = match (req.method(), req.uri().path()){

            // ----------------------------------------------------------------
                (&Method::POST, "/auth/register") => {
                    let res = Response::builder();
                    let whole_body_bytes = hyper::body::aggregate(req.into_body()); //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all futures stream or chunks which is utf8 bytes - since we don't know the end yet, we can't simply stream the chunks as they arrive (cause all futures stream or chunks which are called chunks are arrived asynchronously), so here we do `.await` on the future, waiting on concatenating the full body after all chunks arrived then afterwards the content can be reversed
                    let whole_body_bytes = block_on(whole_body_bytes).unwrap(); //-- future objcets will be created by putting async keyword behind functions, blocks and closure bodies and are bounded to Future trait and can be solved using block_on() method, .await keyword and join!() macro (in async function)
                    

                    
                    let data: serde_json::Value = serde_json::from_reader(whole_body_bytes.reader()).unwrap(); //-- making a serde value from the buffer which is an IO stream of json coming from the client
                    let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json
                    let deserialize_from_json_into_struct: ctx::auth::RegInfo = serde_json::from_str(&json).unwrap();
                    let username = deserialize_from_json_into_struct.username;
                    let phone = deserialize_from_json_into_struct.phone;


                    
                    // TODO - hash password using argon2
                    // TODO - add jwt based token https://blog.logrocket.com/jwt-authentication-in-rust/
                    // TODO - check db mode and then store data in it




                    let response_body = ctx::app::Response::<ctx::auth::RegInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is RegInfo struct
                        data: Some(deserialize_from_json_into_struct), //-- deserialize_from_json_into_struct is of type RegInfo struct 
                        message: ACCESS_GRANTED,
                        status: 200,
                    };

                    let response_body_json = serde_json::to_string(&response_body).unwrap();
                    


                    Ok(
                        res
                            .status(StatusCode::OK)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                            .unwrap() 
                    )
                },
            // ----------------------------------------------------------------
                _ => {
                    let res = Response::builder();
                    let response_body = ctx::app::Response::<Nill>{
                        message: NOTFOUND_ROUTE,
                        data: Some(Nill),
                        status: 404,
                    };
                    let response_body_json = serde_json::to_string(&response_body).unwrap();
                    Ok(
                        res
                            .status(StatusCode::NOT_FOUND) //-- not found route or method not allowed
                            .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
                            .unwrap() 
                    )
                }
            // ---------------------------------------------------------------
            
        };


        Box::pin(async{res}) //-- returning the future response pinned to memory to solve later using .await
        
    }
}
