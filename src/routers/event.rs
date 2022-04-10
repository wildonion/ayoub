









use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::event::{
                                add::main as add_event, 
                                get::{all as get_all_events, single as get_single_events}, 
                                vote::main as cast_vote_event, 
                                expire::main as expire_event, 
                                _404::main as not_found, 
                                phase::insert as insert_phase,
                                simd::main as simd_ops
                            };





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, mut app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{ // NOTE - we've defined the app as mutable cause we want to change the name field later



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::POST, "/event/add")           => {
            app.set_name("/event/add").await;
            add_event(app_storage, app).await
        },
        (&Method::GET, "/event/get/availables") => {
            app.set_name("/event/get/availables").await;
            get_all_events(app_storage, app).await
        },
        (&Method::GET, "/event/get/single") => {
            app.set_name("/event/get/single").await;
            get_single_events(app_storage, app).await
        },
        (&Method::POST, "/event/cast-vote")     => {
            app.set_name("/event/cast-vote").await;
            cast_vote_event(app_storage, app).await
        },
        (&Method::POST, "/event/set-expire")    => {
            app.set_name("/event/set-expire").await;
            expire_event(app_storage, app).await
        },
        (&Method::POST, "/event/simd-ops")      => {
            app.set_name("/event/simd-ops").await;
            simd_ops(app).await
        },
        (&Method::POST, "/event/update/phases/add") => {
            app.set_name("/event/update/phases/add").await;
            insert_phase(app_storage, app).await
        }
        _                                       => not_found().await,
    }

}