





// https://github.com/hyperium/hyper/blob/master/examples/params.rs
// get all (un)successful payments for an event with admin or God access
// get all (un)successful payments for a user with user access
// TODO - after reservation (successful payment):
//          - update role_id and side_id with user_id inside the users collection
//          - insert new player role ability into player_role_ability_info collection
//          - insert the new user_id into the players field of the events collection
// ...




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















// -------------------------------- process payment controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------
pub async fn process_payment(db: Option<&Client>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    
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














