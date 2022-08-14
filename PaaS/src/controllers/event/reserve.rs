








use crate::middlewares;
use crate::utils;
use crate::schemas;
use crate::contexts as ctx;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer and is used for manipulating coming network bytes from the socket
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::Client;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itself cause there is a struct called bson inside the bson.rs file














// https://github.com/hyperium/hyper/blob/master/examples/params.rs
// get all (un)successful payments for an event with admin or God access
// get all (un)successful payments for a user with user access
// TODO - after reservation (successful payment):
//          - update role_id and side_id with user_id inside the users collection which will be the last event game info
//          - role_reveal controller (can only be called by the God of the event game)
//          - insert new player role ability into player_role_ability_info collection
//          - insert new AddPaymentRequest instance into the payments collection
//          - behalf of the user the server will assign the role_id and side_id for the user
// ...









// -------------------------------- mock process payment controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------
pub async fn mock_payment(db: Option<&Client>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    
    api.post("/event/", |req, res| async move{


        let reserve_status = 1; //-- means the payment was successful

        if reserve_status == 1{



            // mock logic here 
            // ...



            let response_body = ctx::app::Response::<ctx::app::Nill>{
                message: NOT_IMPLEMENTED,
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                status: 501,
            };
            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
            Ok(
                res
                    .status(StatusCode::NOT_IMPLEMENTED) //-- not found route or method not allowed
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                    .unwrap()
            )
        } else{
            let response_body = ctx::app::Response::<ctx::app::Nill>{
                message: NOT_IMPLEMENTED,
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                status: 501,
            };
            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
            Ok(
                res
                    .status(StatusCode::NOT_IMPLEMENTED) //-- not found route or method not allowed
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                    .unwrap()
            )
        }








    }).await



}














// -------------------------------- process payment controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------
pub async fn process_payment_request(db: Option<&Client>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    
    api.post("", |req, res| async move{



        let response_body = ctx::app::Response::<ctx::app::Nill>{
            message: NOT_IMPLEMENTED,
            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
            status: 501,
        };
        let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
        Ok(
            res
                .status(StatusCode::NOT_IMPLEMENTED) //-- not found route or method not allowed
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                .unwrap()
        )




    }).await



}














