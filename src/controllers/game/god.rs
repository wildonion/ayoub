





use crate::schemas;
use crate::contexts as ctx;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::Client;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file







// NOTE - the following api is only for God 


// TODO - create groups schema
//          - _id
//          - name
//          - owner (God / user_id)





// -------------------------------- get all users controller
//
// -------------------------------------------------------------------------

pub async fn create_group(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/god/create/group", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
        


        // TODO - need admin (God) access level
        // ...

        
        ////////////////////////////////// DB Ops

        let response_body = ctx::app::Response::<ctx::app::Nill>{
            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
            message: &"none".to_string(), //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
            status: 500,
        };
        let response_body_json = serde_json::to_string(&response_body).unwrap();
        Ok(
            res
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                .unwrap() 
        )

        //////////////////////////////////


    }).await
    
}