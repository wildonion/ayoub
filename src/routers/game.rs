



use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::game::{
                                role::add as add_role, 
                                side::add as add_side, 
                                deck::add as add_deck, 
                                not_found::main as not_found, 
                            };





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, mut app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{ // NOTE - we've defined the app as mutable cause we want to change the name field later



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::POST, "/game/role/add") => {
            app.name = "/game/role/add".to_string();
            add_role(app_storage, app).await
        },
        (&Method::GET, "/game/side/add") => {
            app.name = "/game/side/add".to_string();
            add_side(app_storage, app).await
        },
        (&Method::POST, "/game/deck/add") => {
            app.name = "/game/deck/add".to_string();
            add_deck(app_storage, app).await
        },
        _                                 => not_found().await,
    }


}