









use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::auth::{home, not_found};





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::GET, "/event") => home(app).await,
        _                        => not_found().await
    }

}