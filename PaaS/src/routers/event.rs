




/*
    --------------------------------------------------------------------------
   |                      REGISTER HANDLER FOR EVENT ROUTER
   |--------------------------------------------------------------------------
   |
   |    job    : the following registers a router requested by the client
   |    return : a Result of type either successful or error response
   |
   |

*/





use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::event::{
                                add::main as add_event, 
                                get::{all as get_all_events, player_all as get_all_player_events, single as get_single_event}, 
                                vote::main as cast_vote_event, 
                                expire::main as expire_event, 
                                _404::main as not_found, 
                                phase::insert as insert_phase,
                                reserve::{process_payment_request, mock_reservation},
                                reveal::{role},
                                simd::main as simd_ops
                            };





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, mut app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{ // NOTE - we've defined the app as mutable cause we want to change the name field later



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> so we can haved T on later scopes thus preventing it from moving 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client>
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
        (&Method::GET, "/event/get/all/player") => {
            app.name = "/event/get/all/player".to_string();
            get_all_player_events(app_storage, app).await
        },
        (&Method::POST, "/event/get/single") => {
            app.name = "/event/get/single".to_string();
            get_single_event(app_storage, app).await
        },
        (&Method::POST, "/event/cast-vote")     => {
            app.name = "/event/cast-vote".to_string();
            cast_vote_event(app_storage, app).await
        },
        (&Method::POST, "/event/set-expire")    => {
            app.name = "/event/set-expire".to_string();
            expire_event(app_storage, app).await
        },
        (&Method::POST, "/event/update/phases/add") => {
            app.name = "/event/update/phases/add".to_string();
            insert_phase(app_storage, app).await
        }
        (&Method::POST, "/event/simd")      => {
            app.name = "/event/simd".to_string();
            simd_ops(app).await
        },
        (&Method::POST, "/event/reserve")      => {
            app.name = "/event/reserve".to_string();
            mock_reservation(app_storage, app).await
        },
        (&Method::POST, "/event/reveal/roles")      => {
            app.name = "/event/reveal/roles".to_string();
            role(app_storage, app).await
        },
        _                                       => not_found().await,
    }

}