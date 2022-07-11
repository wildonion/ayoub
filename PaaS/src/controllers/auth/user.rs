





use crate::schemas;
use crate::contexts as ctx;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer and is used for manipulating coming network bytes from the socket
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::Client;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itself cause there is a struct called bson inside the bson.rs file







// NOTE - the following api is only for God 






// -------------------------------- get all users controller
//
// -------------------------------------------------------------------------

pub async fn get_all(db: Option<&Client>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene

    api.post("/auth/user/get/all", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
        


        // TODO - need admin (God) access level
        // ...

        
        ////////////////////////////////// DB Ops

        let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch and deserialize all event infos or documents from BSON into the UserInfo struct
        let mut available_users = schemas::auth::AvailableUsers{
            users: vec![],
        };

        match users.find(None, None).await{
            Ok(mut cursor) => {
                while let Some(event) = cursor.try_next().await.unwrap(){ //-- a mongodb Cursor implements Stream from the futures crate so we can iterate over its future objects by calling try_next() method on cursor which require the cursor to be mutable - reading while awaiting on try_next() method doesn't return None
                    available_users.users.push(event);
                }
                let res = Response::builder(); //-- creating a new response cause we didn't find any available route
                let response_body = ctx::app::Response::<schemas::auth::AvailableUsers>{
                    message: FETCHED,
                    data: Some(available_users),
                    status: 200,
                };
                let response_body_json = serde_json::to_string(&response_body).unwrap();
                Ok(
                    res
                        .status(StatusCode::OK) //-- not found route or method not allowed
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                        .unwrap()
                )
            },
            Err(e) => {
                let response_body = ctx::app::Response::<ctx::app::Nill>{
                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                    message: &e.to_string(), //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                    status: 500,
                };
                let response_body_json = serde_json::to_string(&response_body).unwrap();
                Ok(
                    res
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                        .unwrap() 
                )
            },
        }

        //////////////////////////////////


    }).await
    
}