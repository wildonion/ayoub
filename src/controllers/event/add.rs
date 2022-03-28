





use std::env;
use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body};
use log::info;
use mongodb::{Client, bson::{doc, oid::ObjectId}};










// -------------------------------- add event controller
//
// -------------------------------------------------------------------------
pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    
    api.post("/event/add", |req, res| async move{

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::event::EventAddRequest>(&json){ //-- the generic type of from_str() method is EventAddRequest struct - mapping (deserializing) the json into the EventAddRequest struct
                    Ok(event_info) => { //-- we got the username and password inside the login route


                        ////////////////////////////////// DB Ops
                        
                        let events = db.unwrap().database("ayoub").collection::<schemas::event::EventInfo>("events"); //-- selecting events collection to fetch all event infos into the EventInfo struct
                        match events.find_one(doc!{"title": event_info.clone().title}, None).await.unwrap(){ //-- finding event based on event title
                            Some(event_doc) => { //-- deserializing BSON into the EventInfo struct
                                let response_body = ctx::app::Response::<schemas::event::EventInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is EventInfo struct
                                    data: Some(event_doc), //-- data is an empty &[u8] array
                                    message: FOUND_DOCUMENT, //-- collection found in ayoub document (database)
                                    status: 302,
                                };
                                let response_body_json = serde_json::to_string(&response_body).unwrap();
                                Ok(
                                    res
                                        .status(StatusCode::FOUND)
                                        .header(header::CONTENT_TYPE, "application/json")
                                        .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                                        .unwrap() 
                                )
                            }, 
                            None => { //-- means we didn't find any document related to this title and we have to create a new event
                                let events = db.unwrap().database("ayoub").collection::<schemas::event::EventAddRequest>("events");
                                let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
                                let exp_time = now + env::var("PROPOSAL_EXPIRATION").expect("⚠️ found no event expiration time").parse::<i64>().unwrap();
                                let new_event = schemas::event::EventAddRequest{
                                    title: event_info.title,
                                    content: event_info.content,
                                    creator_wallet_address: Some("0x0000000000000000000000000000000000000000".to_string()),
                                    upvotes: Some(0),
                                    downvotes: Some(0),
                                    voters: Some(vec![]), //-- initializing empty voters
                                    is_expired: Some(false), //-- a event is not expired yet or at initialization
                                    expire_at: Some(exp_time), //-- a event will be expired at
                                    created_at: Some(now),
                                };
                                match events.insert_one(new_event, None).await{
                                    Ok(insert_result) => {
                                        let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                                            data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                                            message: INSERTED,
                                            status: 201,
                                        };
                                        let response_body_json = serde_json::to_string(&response_body).unwrap();
                                        Ok(
                                            res
                                                .status(StatusCode::CREATED)
                                                .header(header::CONTENT_TYPE, "application/json")
                                                .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                                                .unwrap() 
                                        )
                                    },
                                    Err(e) => {
                                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                            message: &e.to_string(), //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                                            status: 406,
                                        };
                                        let response_body_json = serde_json::to_string(&response_body).unwrap();
                                        Ok(
                                            res
                                                .status(StatusCode::NOT_ACCEPTABLE)
                                                .header(header::CONTENT_TYPE, "application/json")
                                                .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                                                .unwrap() 
                                        )
                                    }
                                }
                            },
                        }

                        //////////////////////////////////


                    },
                    Err(e) => {
                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                            message: &e.to_string(), //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                            status: 406,
                        };
                        let response_body_json = serde_json::to_string(&response_body).unwrap();
                        Ok(
                            res
                                .status(StatusCode::NOT_ACCEPTABLE)
                                .header(header::CONTENT_TYPE, "application/json")
                                .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                                .unwrap() 
                        )
                    },
                }
            },
            Err(e) => {
                let response_body = ctx::app::Response::<ctx::app::Nill>{
                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                    message: &e.to_string(), //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                    status: 400,
                };
                let response_body_json = serde_json::to_string(&response_body).unwrap();
                Ok(
                    res
                        .status(StatusCode::BAD_REQUEST)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                        .unwrap() 
                )
            },
        } 
    }).await
}
