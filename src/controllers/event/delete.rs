







use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body};
use log::info;
use mongodb::{Client, bson::{self, doc, oid::ObjectId}}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file










// -------------------------------- delete event controller
//
// -------------------------------------------------------------------------
pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    
    api.post("/event/delete", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::event::DeleteEventRequest>(&json){ //-- the generic type of from_str() method is DeleteEventRequest struct - mapping (deserializing) the json into the DeleteEventRequest struct
                    Ok(delete_info) => { //-- we got the username and password inside the login route

                        
                        ////////////////////////////////// DB Ops
                        
                        let event_id = ObjectId::parse_str(delete_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                        let events = db.unwrap().database("ayoub").collection::<schemas::event::EventInfo>("events"); //-- selecting events collection to fetch all event infos into the EventInfo struct
                        match events.find_one_and_delete(doc!{"_id": event_id}, None).await.unwrap(){ //-- finding event based on event id
                            Some(event_doc) => { //-- deserializing BSON into the EventInfo struct
                                let response_body = ctx::app::Response::<schemas::event::EventInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is EventInfo struct
                                    data: Some(event_doc), //-- data is an empty &[u8] array
                                    message: DELETED, //-- collection found in ayoub database
                                    status: 200,
                                };
                                let response_body_json = serde_json::to_string(&response_body).unwrap();
                                Ok(
                                    res
                                        .status(StatusCode::OK)
                                        .header(header::CONTENT_TYPE, "application/json")
                                        .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                                        .unwrap() 
                                )
                            }, 
                            None => { //-- means we didn't find any document related to this title and we have to tell the user to create a new event
                                let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                    message: NOT_FOUND_DOCUMENT, //-- document not found in database and the user must do a signup
                                    status: 404,
                                };
                                let response_body_json = serde_json::to_string(&response_body).unwrap();
                                Ok(
                                    res
                                        .status(StatusCode::NOT_FOUND)
                                        .header(header::CONTENT_TYPE, "application/json")
                                        .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                                        .unwrap() 
                                )
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