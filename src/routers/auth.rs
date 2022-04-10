









use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::auth::{
                               home::main as home, 
                               check_token::main as check_token, 
                               login::main as login, 
                               signup::main as signup, 
                               _404::main as not_found, 
                               otp_request::main as otp_request, 
                               check_otp::main as check_otp,
                               user::get_all
                            };





pub async fn register(storage: Option<Arc<ctx::app::Storage>>, mut app: ctx::app::Api) -> Result<Response<Body>, hyper::Error>{ // NOTE - we've defined the app as mutable cause we want to change the name field later



    let req = app.req.as_ref().unwrap(); //-- as_ref() method will make a copy by borrowing what's inside the wrapped type, here our wrapped type is Option which as_ref() will convert &Option<T> to Option<&T> 
    let app_storage = match storage.as_ref().unwrap().db.as_ref().unwrap().mode{
        ctx::app::Mode::On => storage.as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached from the server - instance.as_ref() will return the Option<&Client>
        ctx::app::Mode::Off => None, //-- no db is available cause it's off
    };



    match (req.method(), req.uri().path()){
        (&Method::GET, "/auth/home")           => {
            app.set_name("/auth/home").await;
            home(app_storage, app).await
        },
        (&Method::POST, "/auth/login") => {
            app.set_name("/auth/login").await;
            login(app_storage, app).await
        },
        (&Method::POST, "/auth/signup")     => {
            app.set_name("/auth/signup").await;
            signup(app_storage, app).await
        },
        (&Method::POST, "/auth/check-token")    => {
            app.set_name("/auth/check-token").await;
            check_token(app_storage, app).await
        },
        (&Method::POST, "/auth/otp-req")    => {
            app.set_name("/auth/otp-req").await;
            otp_request(app_storage, app).await
        },
        (&Method::POST, "/auth/check-otp")    => {
            app.set_name("/auth/check-otp").await;
            check_otp(app_storage, app).await
        },
        (&Method::POST, "/auth/user/get/all")    => {
            app.set_name("/auth/user/get/all").await;
            get_all(app_storage, app).await
        },
        _                                       => not_found().await,
    }



}