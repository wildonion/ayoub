









use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::event::{add_event, get_all_events, cast_vote_event, expire_event, not_found, simd_ops};





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::POST, "/event/add")           => add_event(app_storage, app).await,
        (&Method::GET, "/event/get/availables") => get_all_events(app_storage, app).await, //-- get all none expired events
        (&Method::POST, "/event/cast-vote")     => cast_vote_event(app_storage, app).await,
        (&Method::POST, "/event/set-expire")    => expire_event(app_storage, app).await,
        (&Method::POST, "/event/simd-ops")      => simd_ops(app).await,
        _                                       => not_found(app).await
    }

}