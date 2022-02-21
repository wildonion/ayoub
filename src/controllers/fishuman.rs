






use crate::contexts as ctx;
use crate::schemas;
use crate::utils;
use crate::constants::{ACCESS_GRANTED, ACCESS_DENIED,
                      NOTFOUND_ROUTE, DO_LOGIN, DO_SIGNUP, 
                      WELCOME, UNAUTHORISED, REGISTERED};
use std::thread;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;













// -------------------------------- not found controller
//
// -------------------------------------------------------------------------
pub async fn not_found() -> Result<hyper::Response<Body>, hyper::Error>{
    let res = Response::builder(); //-- creating a new response cause we didn't find any available route
    let response_body = ctx::app::Response::<ctx::app::Nill>{
        message: NOTFOUND_ROUTE,
        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
        status: 404,
    };
    let response_body_json = serde_json::to_string(&response_body).unwrap();
    Ok(
        res
            .status(StatusCode::NOT_FOUND) //-- not found route or method not allowed
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
            .unwrap()
    )
}










// -------------------------------- home controller
//
// -------------------------------------------------------------------------
pub async fn home(api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    api.get("/event", |req, res| async move{
        let thread = thread::spawn(|| async move{ //-- the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) for with type either () or a especific type
        info!("inside the native thread");
            let async_task = tokio::spawn(async move{ //-- spawning async task to solve it on the background using tokio green threads based on its event loop model - 
                info!("inside tokio green thread");
                ////////
                // ....
                ////////
            });
        });
        let response_body = ctx::app::Response::<ctx::app::Nill>{
            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
            message: WELCOME,
            status: 200,
        };
        let response_body_json = serde_json::to_string(&response_body).unwrap();
        Ok(
            res
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
                .unwrap()
        )
    }).await
}