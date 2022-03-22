









use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::event::{add_event, get_all_events, cast_vote_event, expire_event, not_found, simd_ops};





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, mut app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{ // NOTE - we've defined the app as mutable cause we want to change the name field later



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::POST, "/event/add")           => {
            app.name = "/event/add".to_string();
            add_event(app_storage, app).await
        },
        (&Method::GET, "/event/get/availables") => {
            app.name = "/event/get/availables".to_string();
            get_all_events(app_storage, app).await
        },
        (&Method::POST, "/event/cast-vote")     => {
            app.name = "/event/cast-vote".to_string();
            cast_vote_event(app_storage, app).await
        },
        (&Method::POST, "/event/set-expire")    => {
            app.name = "/event/set-expire".to_string();
            expire_event(app_storage, app).await
        },
        (&Method::POST, "/event/simd-ops")      => {
            app.name = "/event/simd-ops".to_string();
            simd_ops(app).await
        },
        _                                       => not_found().await,
    }

}