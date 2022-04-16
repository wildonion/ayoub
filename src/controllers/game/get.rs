




use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use mongodb::Client;
use log::info;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file









// -------------------------------- get all groups controller
//
// -------------------------------------------------------------------------
pub async fn all_groups(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    
    api.post("/game/get/group/all", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that    


        ////////////////////////////////// DB Ops
                        
        let groups = db.unwrap().database("ayoub").collection::<schemas::game::GroupInfo>("groups"); //-- selecting groups collection to fetch and deserialize all groups infos or documents from BSON into the GroupInfo struct
        let mut available_groups = schemas::game::AvailableGroups{
            groups: vec![],
        };

        match groups.find(None, None).await{
            Ok(mut cursor) => {
                while let Some(group) = cursor.try_next().await.unwrap(){ //-- calling try_next() method on cursor needs the cursor to be mutable - reading while awaiting on try_next() method doesn't return None
                    available_groups.groups.push(group);
                }
                let res = Response::builder(); //-- creating a new response cause we didn't find any available route
                let response_body = ctx::app::Response::<schemas::game::AvailableGroups>{
                    message: FETCHED,
                    data: Some(available_groups),
                    status: 200,
                };
                let response_body_json = serde_json::to_string(&response_body).unwrap();
                Ok(
                    res
                        .status(StatusCode::OK) //-- not found route or method not allowed
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
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
                        .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                        .unwrap() 
                )
            },
        }

        //////////////////////////////////
        
        
    }).await
}