




/*
    --------------------------------------------------------------------------
   |                      REGISTER HANDLER FOR AUTH ROUTER
   |--------------------------------------------------------------------------
   |
   |    job    : the following registers a router requested by the client
   |    return : a Result of type either successful or error response
   |
   |

*/




use crate::middlewares;
use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::auth::{
                               home::main as home, 
                               check_token::main as check_token, 
                               login::main as login, 
                               signup::{main as signup, register_god}, 
                               _404::main as not_found, 
                               otp_request::main as otp_request, 
                               check_otp::main as check_otp,
                               user::get_all
                            };





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, mut app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{ // NOTE - we've defined the app as mutable cause we want to change the name field later



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> so we can haved T on later scopes thus preventing it from moving 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::GET, "/auth/home")           => {
            app.name = "/auth/home".to_string();
            home(app_storage, app).await
        },
        (&Method::POST, "/auth/login") => {
            app.name = "/auth/login".to_string();
            login(app_storage, app).await
        },
        (&Method::POST, "/auth/signup")     => {
            app.name = "/auth/signup".to_string();
            signup(app_storage, app).await
        },
        (&Method::POST, "/auth/signup/new-god")     => {
            app.name = "/auth/signup/new-god".to_string();
            register_god(app_storage, app).await
        },
        (&Method::POST, "/auth/check-token")    => {
            app.name = "/auth/check-token".to_string();
            check_token(app_storage, app).await
        },
        (&Method::POST, "/auth/otp-req")    => {
            app.name = "/auth/otp-req".to_string();
            otp_request(app_storage, app).await
        },
        (&Method::POST, "/auth/check-otp")    => {
            app.name = "/auth/check-otp".to_string();
            check_otp(app_storage, app).await
        },
        (&Method::POST, "/auth/user/get/all")    => {
            app.name = "/auth/user/get/all".to_string();
            get_all(app_storage, app).await
        },
        (&Method::OPTIONS, "/")    => {
            middlewares::cors::send_preflight_response().await
        },
        _                                       => not_found().await,
    }



}