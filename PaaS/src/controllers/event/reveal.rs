






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
pub async fn role(db: Option<&Client>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    
    api.post("/event/reveal/roles", |req, res| async move{

        match middlewares::auth::pass(req).await{
            Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                                
                
        
                let _id = token_data.claims._id;
                let username = token_data.claims.username;
                let access_level = token_data.claims.access_level;
        
                
                
                if middlewares::auth::user::exists(db, _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                    if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                                let data: serde_json::Value = value;
                                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                                match serde_json::from_str::<schemas::event::GetEventRequest>(&json){ //-- the generic type of from_str() method is GetEventRequest struct - mapping (deserializing) the json string into the GetEventRequest struct
                                    Ok(event_info) => {

                                        
                                        ////////////////////////////////// DB Ops

                                        let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting events collection to fetch and deserialize all user infos or documents from BSON into the UserInfo struct
                                        let event_id = ObjectId::parse_str(event_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                        let events = db.unwrap().database("ayoub").collection::<schemas::event::EventInfo>("events"); //-- selecting events collection to fetch and deserialize all event infos or documents from BSON into the EventInfo struct
                                        match events.find_one(doc! { "_id": event_id, "is_expired": false }, None).await.unwrap(){ //-- getting a none expired event
                                            Some(event_doc) => {

                                                for p in event_doc.players{
                                                    let random_role_id = "mongodb-object-id"; // TODO
                                                    let random_side_id = "mongodb-object-id"; // TODO
                                                    let serialized_random_side_id = ObjectId::parse_str(random_side_id).unwrap(); //-- generating mongodb object id from the id string
                                                    let serialized_random_role_id = ObjectId::parse_str(random_role_id).unwrap(); //-- generating mongodb object id from the id string
                                                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                                    match users.find_one_and_update(doc! { "_id": p._id }, doc!{"$set": {"role_id": serialized_random_role_id, "side_id": serialized_random_side_id, "updated_at": Some(now)}}, None).await.unwrap(){ //-- finding user based on _id
                                                        Some(user_doc) => { //-- we updated the users collection successfully now we have to update each player inside the current event 
                                                            

                                                            p.role_id = Some(serialized_random_role_id);
                                                            p.side_id = Some(serialized_random_side_id);
                                                            


                                                            // TODO - update the players field inside the event_doc with updated_players
                                                            // TODO - insert new player role ability for the reserved event into player_role_ability_info collection
                                                            // ...


                                                            let response_body = ctx::app::Response::<schemas::event::EventInfo>{
                                                                message: FETCHED,
                                                                data: Some(event_doc),
                                                                status: 200,
                                                            };
                                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                                            Ok(
                                                                res
                                                                    .status(StatusCode::OK)
                                                                    .header(header::CONTENT_TYPE, "application/json")
                                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                                                                    .unwrap()
                                                            )
                                                            


                                                        },
                                                        None => { //-- means we didn't find any document related to this user_id and simply we return the unmatched player info
                                                            let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                                                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                                                message: NOT_FOUND_PLAYER, //-- document not found in database and the user must do a signup
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

                                                










                                                let response_body = ctx::app::Response::<schemas::event::EventInfo>{
                                                    message: FETCHED,
                                                    data: Some(event_doc),
                                                    status: 200,
                                                };
                                                let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                                Ok(
                                                    res
                                                        .status(StatusCode::OK)
                                                        .header(header::CONTENT_TYPE, "application/json")
                                                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                                                        .unwrap()
                                                )
                                            },
                                            None => { //-- means we didn't find any document related to this user_id and we have to tell the user do a signup
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

                                        //////////////////////////////////


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
    }).await

}