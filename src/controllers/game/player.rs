








use crate::contexts as ctx;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use mongodb::Client;






// TODO - update phase APIs
// TODO - must update the role_id, status, side_id inside the users collection
// TODO - role reservation by God and the server
// ...







// -------------------------------- update player role controller
//
// -------------------------------------------------------------------------

pub async fn update_role(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
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










// -------------------------------- update player side controller
//
// -------------------------------------------------------------------------

pub async fn update_side(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

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









// -------------------------------- update player role ability controller
//
// -------------------------------------------------------------------------

pub async fn update_role_ability(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
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







// -------------------------------- update player status controller
//
// -------------------------------------------------------------------------

pub async fn update_status(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
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







// -------------------------------- chain player to another player controller
//
// ----------------------------------------------------------------------------------

pub async fn chain_to_another_player(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
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