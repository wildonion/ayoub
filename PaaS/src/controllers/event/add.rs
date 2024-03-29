





use crate::middlewares;
use crate::utils;
use crate::schemas;
use crate::contexts as ctx;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer and is used for manipulating coming network bytes from the socket
use hyper::{header, StatusCode, Body, Response, Request};
use log::info;
use mongodb::Client;
use mongodb::bson::{self, oid::ObjectId, doc};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use routerify_multipart::RequestMultipartExt; //-- self referes to the bson struct itself cause there is a struct called bson inside the bson.rs file
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

















// -------------------------------- upload event image controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn upload_img(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Client>().unwrap().to_owned();
    
    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
    
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    
                    let event_id = format!("{}", req.param("eventId").unwrap()); //-- we must create the url param using format!() since this macro will borrow the req object and doesn't move it so we can access the req object later to handle incoming multipart data
                    let event_object_id = ObjectId::from_str(&event_id).unwrap(); //-- always pass by reference to not to lose ownership of the type

                    match req.into_multipart(){ //-- converting the request object into multipart content type to get the inomcing IO streaming of bytes of the uploaded file - some where the RequestMultipartExt trait has implemented for the request object so we can call the into_multipart() method on the req object
                        Ok(payload) => {


                            ////////////////////////////////// DB Ops
                            
                            let events = db.clone().database(&db_name).collection::<schemas::event::EventInfo>("events"); //-- selecting events collection to fetch all event infos into the EventInfo struct
                            let update_option = FindOneAndUpdateOptions::builder().return_document(Some(ReturnDocument::After)).build();
                            match events.find_one(doc!{"_id": event_object_id}, None).await.unwrap(){
                                Some(event_doc) => {
                                    if event_doc.clone().group_info.unwrap().god_id.unwrap() == _id.unwrap().to_string() || access_level == DEV_ACCESS{
                                        let filepath = utils::upload_asset(EVENT_UPLOAD_PATH, payload, &event_id).await; //-- passing the incoming multipart payload to build the image from its IO stream utf8 bytes future object 
                                        let upload_instance = utils::UploadFile{
                                            name: filepath.clone(),
                                            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                                        };
                                        info!("{} - uploaded instance {:?}", chrono::Local::now().naive_local(), upload_instance);
                                        
                                        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
                                        match events.find_one_and_update(doc!{"_id": event_object_id}, doc!{
                                            "$set": {
                                                "image_path": filepath.unwrap(),
                                                "updated_at": Some(now),
                                            }}, Some(update_option)).await.unwrap(){
                                                Some(event_doc) => { 
                                                    let response_body = ctx::app::Response::<schemas::event::EventInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is EventInfo struct
                                                        data: Some(event_doc),
                                                        message: UPLOADED,
                                                        status: 200,
                                                    };
                                                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                                    Ok(
                                                        res
                                                            .status(StatusCode::FOUND)
                                                            .header(header::CONTENT_TYPE, "application/json")
                                                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                            .unwrap() 
                                                    )        
                                                }, 
                                                None => { //-- if we found a group we'll never be in here; but for sure we must handle any sudden error :)
                                                    let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                                        message: NOT_FOUND_DOCUMENT, //-- document not found in database and the user must do a signup
                                                        status: 404,
                                                    };
                                                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                                    Ok(
                                                        res
                                                            .status(StatusCode::NOT_FOUND)
                                                            .header(header::CONTENT_TYPE, "application/json")
                                                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                            .unwrap() 
                                                    )
                                                },
                                            }
                                        } else{ //-- only the god of the group can update the group image
                                            let response_body = ctx::app::Response::<ctx::app::Nill>{
                                                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                                message: ACCESS_DENIED,
                                                status: 403,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::BAD_REQUEST)
                                                    .header(header::CONTENT_TYPE, "application/json")
                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                    .unwrap() 
                                            )   
                                        }
                                },
                                None => { //-- no group found with the passed in id in url param
                                    let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                        message: NOT_FOUND_DOCUMENT,
                                        status: 404,
                                    };
                                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                    Ok(
                                        res
                                            .status(StatusCode::NOT_FOUND)
                                            .header(header::CONTENT_TYPE, "application/json")
                                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
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
                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                            Ok(
                                res
                                    .status(StatusCode::BAD_REQUEST)
                                    .header(header::CONTENT_TYPE, "application/json")
                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                    .unwrap() 
                            )
                        },
                    }
                } else{ //-- access denied for this user with none admin and dev access level
                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                        message: ACCESS_DENIED,
                        status: 403,
                    };
                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                    Ok(
                        res
                            .status(StatusCode::BAD_REQUEST)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                            .unwrap() 
                    )
                }
            } else{ //-- user doesn't exist :(
                let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                    message: DO_SIGNUP, //-- document not found in database and the user must do a signup
                    status: 404,
                };
                let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                Ok(
                    res
                        .status(StatusCode::NOT_FOUND)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                        .unwrap() 
                )
            }
        },
        Err(e) => {
            let response_body = ctx::app::Response::<ctx::app::Nill>{
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                message: &e, //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                status: 500,
            };
            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
            Ok(
                res
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                    .unwrap() 
            )
        },
    }

}












// -------------------------------- add event controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------
pub async fn main(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{
    
     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Client>().unwrap().to_owned();

    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            

            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;


            let db_to_pass = db.clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::event::AddEventRequest>(&json){ //-- the generic type of from_str() method is AddEventRequest struct - mapping (deserializing) the json string into the AddEventRequest struct
                                Ok(event_info) => { //-- we got the username and password inside the login route

                                    if event_info.group_info.clone().unwrap().god_id.unwrap() == _id.unwrap().to_string(){

                                        ////////////////////////////////// DB Ops
                                    
                                        let update_option = FindOneAndUpdateOptions::builder().return_document(Some(ReturnDocument::After)).build();
                                        let events = db.clone().database(&db_name).collection::<schemas::event::EventInfo>("events"); //-- selecting events collection to fetch all event infos into the EventInfo struct
                                        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
                                        match events.find_one_and_update(doc!{"title": event_info.clone().title}, doc!{
                                            "$set": {
                                                "title": bson::to_bson(&event_info.title).unwrap(),
                                                "content": bson::to_bson(&event_info.content).unwrap(),
                                                "deck_id": bson::to_bson(&event_info.deck_id).unwrap(), //-- it's ObjectId of the selected deck but string-ed!
                                                "entry_price": bson::to_bson(&event_info.entry_price).unwrap(),
                                                "group_info": bson::to_bson(&event_info.group_info).unwrap(),
                                                "creator_wallet_address": Some(bson::to_bson(&event_info.creator_wallet_address).unwrap()),
                                                "phases": Some(bson::to_bson(&event_info.phases).unwrap()), //-- initializing empty vector of phases
                                                "max_players": bson::to_bson(&event_info.max_players).unwrap(), //-- this is the maximum players that an event can have
                                                "players": Some(bson::to_bson(&event_info.players).unwrap()), //-- there are no participant yet for this event
                                                "started_at": Some(bson::to_bson(&event_info.started_at).unwrap()),
                                                "updated_at": Some(now),
                                            }  
                                        }, Some(update_option)).await.unwrap(){ //-- finding event based on event title
                                            Some(event_doc) => { //-- deserializing BSON into the EventInfo struct
                                                let response_body = ctx::app::Response::<schemas::event::EventInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is EventInfo struct
                                                    data: Some(event_doc),
                                                    message: FOUND_DOCUMENT_UPDATE, //-- collection found in ayoub database
                                                    status: 302,
                                                };
                                                let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                                Ok(
                                                    res
                                                        .status(StatusCode::FOUND)
                                                        .header(header::CONTENT_TYPE, "application/json")
                                                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                        .unwrap() 
                                                )
                                            }, 
                                            None => { //-- means we didn't find any document related to this title and we have to create a new event
                                                let events = db.clone().database(&db_name).collection::<schemas::event::AddEventRequest>("events");
                                                let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
                                                let exp_time = now + env::var("EVENT_EXPIRATION").expect("⚠️ found no event expiration time").parse::<i64>().unwrap();
                                                let new_event = schemas::event::AddEventRequest{
                                                    title: event_info.title,
                                                    content: event_info.content,
                                                    deck_id: event_info.deck_id, //-- it's ObjectId of the selected deck but string-ed!
                                                    entry_price: event_info.entry_price,
                                                    group_info: event_info.group_info,
                                                    image_path: Some("".to_string()),
                                                    creator_wallet_address: Some("0x0000000000000000000000000000000000000000".to_string()),
                                                    upvotes: Some(0),
                                                    downvotes: Some(0),
                                                    voters: Some(vec![]), //-- initializing empty voters
                                                    phases: Some(vec![]), //-- initializing empty vector of phases
                                                    max_players: event_info.max_players, //-- this is the maximum players that an event can have
                                                    players: Some(vec![]), //-- there are no participant yet for this event
                                                    is_expired: Some(false), //-- a event is not expired yet or at initialization
                                                    is_locked: Some(false), //-- a event is not locked yet or at initialization
                                                    started_at: event_info.started_at,
                                                    expire_at: Some(exp_time), //-- a event will be expired at
                                                    created_at: Some(now),
                                                    updated_at: Some(now),
                                                };
                                                match events.insert_one(new_event, None).await{
                                                    Ok(insert_result) => {
                                                        let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                                                            data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                                                            message: INSERTED,
                                                            status: 201,
                                                        };
                                                        let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                                        Ok(
                                                            res
                                                                .status(StatusCode::CREATED)
                                                                .header(header::CONTENT_TYPE, "application/json")
                                                                .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                                .unwrap() 
                                                        )
                                                    },
                                                    Err(e) => {
                                                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                                                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                                            message: &e.to_string(), //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                                                            status: 406,
                                                        };
                                                        let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                                        Ok(
                                                            res
                                                                .status(StatusCode::NOT_ACCEPTABLE)
                                                                .header(header::CONTENT_TYPE, "application/json")
                                                                .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                                .unwrap() 
                                                        )
                                                    }
                                                }
                                            },
                                        }

                                        //////////////////////////////////

                                    } else{
                                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                            message: ACCESS_DENIED,
                                            status: 403,
                                        };
                                        let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                        Ok(
                                            res
                                                .status(StatusCode::BAD_REQUEST)
                                                .header(header::CONTENT_TYPE, "application/json")
                                                .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                .unwrap() 
                                        )
                                    }
                                },
                                Err(e) => {
                                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                        message: &e.to_string(), //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                                        status: 406,
                                    };
                                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                    Ok(
                                        res
                                            .status(StatusCode::NOT_ACCEPTABLE)
                                            .header(header::CONTENT_TYPE, "application/json")
                                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                            .unwrap_or(hyper::Response::default()) 
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
                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                            Ok(
                                res
                                    .status(StatusCode::BAD_REQUEST)
                                    .header(header::CONTENT_TYPE, "application/json")
                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                    .unwrap() 
                            )
                        },
                    }
                } else{ //-- access denied for this user with none admin and dev access level
                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                        message: ACCESS_DENIED,
                        status: 403,
                    };
                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                    Ok(
                        res
                            .status(StatusCode::BAD_REQUEST)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                            .unwrap() 
                    )
                }

            } else{ //-- user doesn't exist :(
                let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                    message: DO_SIGNUP, //-- document not found in database and the user must do a signup
                    status: 404,
                };
                let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                Ok(
                    res
                        .status(StatusCode::NOT_FOUND)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                        .unwrap() 
                )
            }
        },
        Err(e) => {
            let response_body = ctx::app::Response::<ctx::app::Nill>{
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                message: &e, //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                status: 500,
            };
            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
            Ok(
                res
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                    .unwrap() 
            )
        },
    }

}
