



use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::game::{
                                role::{add as add_role, all as get_roles, disable as diable_role}, 
                                deck::{add as add_deck, all as get_decks, disable as diable_deck},
                                side::{add as add_side, all as get_sides, disable as diable_side}, 
                                player::{update_role_ability, chain_to_another_player, update_role, update_side, update_status, get_single}, 
                                _404::main as not_found, 
                            };





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, mut app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{ // NOTE - we've defined the app as mutable cause we want to change the name field later



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::POST, "/game/role/add") => {
            app.set_name("/game/role/add").await;
            add_role(app_storage, app).await
        },
        (&Method::POST, "/game/role/get/availables") => {
            app.set_name("/game/role/get/availables").await;
            get_roles(app_storage, app).await
        },
        (&Method::POST, "/game/role/diable") => {
            app.set_name("/game/role/diable").await;
            diable_role(app_storage, app).await
        },
        (&Method::POST, "/game/deck/add") => {
            app.set_name("/game/deck/add").await;
            add_deck(app_storage, app).await
        },
        (&Method::POST, "/game/deck/get/availables") => {
            app.set_name("/game/deck/get/availables").await;
            get_decks(app_storage, app).await
        },
        (&Method::POST, "/game/deck/diable") => {
            app.set_name("/game/deck/diable").await;
            diable_deck(app_storage, app).await
        },
        (&Method::GET, "/game/side/add") => {
            app.set_name("/game/side/add").await;
            add_side(app_storage, app).await
        },
        (&Method::GET, "/game/side/get/availables") => {
            app.set_name("/game/side/get/availables").await;
            get_sides(app_storage, app).await
        },
        (&Method::POST, "/game/side/diable") => {
            app.set_name("/game/side/diable").await;
            diable_side(app_storage, app).await
        },
        (&Method::POST, "/game/player/update/role") => {
            app.set_name("/game/player/update/role").await;
            update_role(app_storage, app).await
        },
        (&Method::POST, "/game/player/update/side") => {
            app.set_name("/game/player/update/side").await;
            update_side(app_storage, app).await
        },
        (&Method::POST, "/game/player/update/status") => {
            app.set_name("/game/player/update/status").await;
            update_status(app_storage, app).await
        },
        (&Method::POST, "/game/player/update/role-ability") => {
            app.set_name("/game/player/update/role-ability").await;
            update_role_ability(app_storage, app).await
        },
        (&Method::POST, "/game/player/chain") => {
            app.set_name("/game/player/chain").await;
            chain_to_another_player(app_storage, app).await
        },
        (&Method::POST, "/game/player/get/single") => {
            app.set_name("/game/player/get/single").await;
            get_single(app_storage, app).await
        },
        _                                 => not_found().await,
    }


}