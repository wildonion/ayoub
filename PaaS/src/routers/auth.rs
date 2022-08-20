




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




use routerify::Router;
use routerify_cors::enable_cors_all;
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





pub async fn register() -> Router<Body, hyper::Error>{  




    Router::builder()
        .middleware(enable_cors_all())
        .get("/auth/home", home)
        .post("/auth/login", login)
        .post("/auth/signup",signup)
        .post("/auth/signup/new-god", register_god)
        .post("/auth/check-token", check_token)
        .post("/auth/otp-req", otp_request)
        .post("/auth/check-otp", check_otp)
        .post("/auth/user/get/all", get_all)
        .options("/", middlewares::cors::send_preflight_response)
        .build()
        .unwrap()




}