




/*
    --------------------------------------------------------------------------
   |                      REGISTER HANDLER FOR AUTH ROUTER
   |--------------------------------------------------------------------------
   |
   |    job    : the following registers a router requested by the client
   |    return : a Router of type either hyper response body or error response
   |
   |
   |
   | we don't need to have one response object for each router and we can build
   | a new one inside the body of each router since rust doesn't support garbage
   | collection rule and each response object will be dropped once each router 
   | router body scope gets ended.
   |
   |
   | instead of initializing the app_storage inside each router api we've 
   | initialized it only once per router to move it between each router api.
   | 

*/




use std::env;
use mongodb::Client;
use routerify::{Router, Middleware};
use routerify_cors::enable_cors_all;
use crate::middlewares;
use crate::contexts as ctx;
use crate::constants::*;
use hyper::{header, Body, Response, StatusCode};
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

    
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let app_storage = Client::with_uri_str(&db_addr).await.unwrap();

    ////////
    // NOTE - only the request object must be passed through each handler
    ////////`   

    Router::builder()
        .data(app_storage) //-- sharing the initialized app_storage between routers' threads
        .middleware(Middleware::post(middlewares::cors::allow))
        // .middleware(enable_cors_all()) //-- enable CORS middleware on the incoming request then pass it to the next middleware
        .middleware(Middleware::pre(middlewares::logging::logger)) //-- enable logging middleware on the incoming request then pass it to the next middleware
        .get("/page", |req| async move{
            let res = Response::builder(); //-- creating a new response cause we didn't find any available route
            let response_body = ctx::app::Response::<ctx::app::Nill>{
                message: WELCOME,
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                status: 200,
            };
            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
            Ok(
                res
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                    .unwrap()
            )
        })
        .get("/home", home)
        .post("/login", login)
        .post("/signup",signup)
        .post("/signup/new-god", register_god)
        .post("/check-token", check_token)
        .post("/otp-req", otp_request)
        .post("/check-otp", check_otp)
        .post("/user/get/all", get_all)
        .any(not_found) //-- handling 404 request
        // .options("/*", middlewares::cors::check_preflight)
        .build()
        .unwrap()



}