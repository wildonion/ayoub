






use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::{Client, bson::{doc, oid::ObjectId}};











// -------------------------------- get all events controller
//
// -------------------------------------------------------------------------
pub async fn all(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    
    api.post("/event/get/availables", |req, res| async move{    


        ////////////////////////////////// DB Ops
                        
        let filter = doc! { "is_expired": false }; //-- filtering all none expired events
        let events = db.unwrap().database("ayoub").collection::<schemas::event::EventInfo>("events"); //-- selecting events collection to fetch and deserialize all event infos or documents from BSON into the EventInfo struct
        let mut available_events = schemas::event::AvailableEvents{
            events: vec![],
        };

        match events.find(filter, None).await{
            Ok(mut cursor) => {
                while let Some(event) = cursor.try_next().await.unwrap(){ //-- calling try_next() method on cursor needs the cursor to be mutable - reading while awaiting on try_next() method doesn't return None
                    available_events.events.push(event);
                }
                let res = Response::builder(); //-- creating a new response cause we didn't find any available route
                let response_body = ctx::app::Response::<schemas::event::AvailableEvents>{
                    message: FETCHED,
                    data: Some(available_events),
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