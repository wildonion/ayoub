






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
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itself cause there is a struct called bson inside the bson.rs file
use hyper::http::Uri;
use std::env;




// NOTE - following are in-game api calls which will be called by the God of the game






// -------------------------------- update player role controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn update_role(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();
    
    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
    
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::auth::UserRoleUpdateRequest>(&json){ //-- the generic type of from_str() method is UserRoleUpdateRequest struct - mapping (deserializing) the json string into the UserRoleUpdateRequest struct
                                Ok(update_info) => { //-- we got the username and password inside the login route
                                    

                                    ////////////////////////////////// DB Ops

                                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let role_id = ObjectId::parse_str(update_info.role_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                    let users = db.clone().unwrap().database(&db_name).collection::<schemas::auth::UserInfo>("users"); //-- connecting to users collection to update the role_id field - we want to deserialize all user bsons into the UserInfo struct
                                    match users.find_one_and_update(doc!{"_id": user_id}, doc!{"$set": {"role_id": role_id, "updated_at": Some(now)}}, None).await.unwrap(){
                                        Some(user_doc) => {
                                            let user_info = schemas::auth::UserUpdateResponse{
                                                username: user_doc.username,
                                                phone: user_doc.phone,
                                                access_level: user_doc.access_level,
                                                status: user_doc.status,
                                                role_id: user_doc.role_id, // NOTE - updated
                                                side_id: user_doc.side_id,
                                                created_at: user_doc.created_at,
                                                updated_at: Some(now), // NOTE - updated
                                                last_login_time: user_doc.last_login_time,
                                            };
                                            let response_body = ctx::app::Response::<schemas::auth::UserUpdateResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is UserUpdateResponse struct
                                                data: Some(user_info),
                                                message: UPDATED,
                                                status: 200,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::OK)
                                                    .header(header::CONTENT_TYPE, "application/json")
                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                    .unwrap() 
                                            )
                                        },
                                        None => { //-- means we didn't find any document related to this user_id and we have to tell the user do a signup
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
}










// -------------------------------- update player side controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn update_side(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();

    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
    
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::auth::UserSideUpdateRequest>(&json){ //-- the generic type of from_str() method is UserSideUpdateRequest struct - mapping (deserializing) the json string into the UserRoleUpdateRequest struct
                                Ok(update_info) => { //-- we got the username and password inside the login route

                                    ////////////////////////////////// DB Ops

                                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let side_id = ObjectId::parse_str(update_info.side_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                    let users = db.clone().unwrap().database(&db_name).collection::<schemas::auth::UserInfo>("users"); //-- connecting to users collection to update the side_id field - we want to deserialize all user bsons into the UserInfo struct
                                    match users.find_one_and_update(doc!{"_id": user_id}, doc!{"$set": {"side_id": side_id, "updated_at": Some(now)}}, None).await.unwrap(){
                                        Some(user_doc) => {
                                            let user_info = schemas::auth::UserUpdateResponse{
                                                username: user_doc.username,
                                                phone: user_doc.phone,
                                                access_level: user_doc.access_level,
                                                status: user_doc.status,
                                                role_id: user_doc.role_id,
                                                side_id: user_doc.side_id, // NOTE - updated
                                                created_at: user_doc.created_at,
                                                updated_at: Some(now), // NOTE - updated
                                                last_login_time: user_doc.last_login_time,
                                            };
                                            let response_body = ctx::app::Response::<schemas::auth::UserUpdateResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is UserUpdateResponse struct
                                                data: Some(user_info),
                                                message: UPDATED,
                                                status: 200,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::OK)
                                                    .header(header::CONTENT_TYPE, "application/json")
                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                    .unwrap() 
                                            )
                                        },
                                        None => { //-- means we didn't find any document related to this user_id and we have to tell the user do a signup
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

}







// -------------------------------- update player status controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn update_status(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{
    
     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();
    
    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
    
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::auth::UserStatusUpdateRequest>(&json){ //-- the generic type of from_str() method is UserStatusUpdateRequest struct - mapping (deserializing) the json string into the UserRoleUpdateRequest struct
                                Ok(update_info) => { //-- we got the username and password inside the login route
                                    
                                    ////////////////////////////////// DB Ops

                                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let status = bson::to_bson(&update_info.status).unwrap(); //-- we have to serialize the status to BSON Document object in order to update the mentioned field inside the collection
                                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                    let users = db.clone().unwrap().database(&db_name).collection::<schemas::auth::UserInfo>("users"); //-- connecting to users collection to update the status field - we want to deserialize all user bsons into the UserInfo struct
                                    match users.find_one_and_update(doc!{"_id": user_id}, doc!{"$set": {"status": status, "updated_at": Some(now)}}, None).await.unwrap(){
                                        Some(user_doc) => {
                                            let user_info = schemas::auth::UserUpdateResponse{
                                                username: user_doc.username,
                                                phone: user_doc.phone,
                                                access_level: user_doc.access_level,
                                                status: user_doc.status, // NOTE - updated
                                                role_id: user_doc.role_id,
                                                side_id: user_doc.side_id,
                                                created_at: user_doc.created_at,
                                                updated_at: Some(now), // NOTE - updated
                                                last_login_time: user_doc.last_login_time,
                                            };
                                            let response_body = ctx::app::Response::<schemas::auth::UserUpdateResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is UserUpdateResponse struct
                                                data: Some(user_info),
                                                message: UPDATED,
                                                status: 200,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::OK)
                                                    .header(header::CONTENT_TYPE, "application/json")
                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                    .unwrap() 
                                            )
                                        },
                                        None => { //-- means we didn't find any document related to this user_id and we have to tell the user do a signup
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

}









// -------------------------------- update player role ability controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn update_role_ability(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();
    
    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
    
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::game::UpdatePlayerRoleAbilityRequest>(&json){ //-- the generic type of from_str() method is UpdatePlayerRoleAbilityRequest struct - mapping (deserializing) the json string into the UserRoleUpdateRequest struct
                                Ok(update_info) => { //-- we got the username and password inside the login route
                                    
                                    ////////////////////////////////// DB Ops
                
                                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let event_id = ObjectId::parse_str(update_info.event_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let role_id = ObjectId::parse_str(update_info.role_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                    let current_ability = bson::to_bson(&update_info.current_ability).unwrap(); //-- we have to serialize the current_ability to BSON Document object in order to update the mentioned field inside the collection
                                    let player_roles_info = db.clone().unwrap().database(&db_name).collection::<schemas::game::PlayerRoleAbilityInfo>("player_role_ability_info"); //-- connecting to player_role_ability_info collection to update the current_ability field - we want to deserialize all user bsons into the PlayerRoleAbilityInfo struct
                                    match player_roles_info.find_one_and_update(doc!{"user_id": user_id, "event_id": event_id, "role_id": role_id}, doc!{"$set": {"current_ability": Some(current_ability), "updated_at": Some(now)}}, None).await.unwrap(){
                                        Some(user_doc) => {
                                            let response_body = ctx::app::Response::<schemas::game::PlayerRoleAbilityInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is PlayerRoleAbilityInfo struct
                                                data: Some(user_doc),
                                                message: UPDATED,
                                                status: 200,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::OK)
                                                    .header(header::CONTENT_TYPE, "application/json")
                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                    .unwrap() 
                                            )
                                        },
                                        None => { //-- means we didn't find any document related to this user_id and we have to tell the user do a signup
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

}











// -------------------------------- chain player to another player controller
// ➝ Return : Hyper Response Body or Hyper Error
// ----------------------------------------------------------------------------------

pub async fn chain_to_another_player(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    
     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();

    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
    
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::game::InsertPlayerChainToRequest>(&json){ //-- the generic type of from_str() method is InsertPlayerChainToRequest struct - mapping (deserializing) the json string into the UserRoleUpdateRequest struct
                                Ok(update_info) => { //-- we got the username and password inside the login route
                                    
                                    ////////////////////////////////// DB Ops

                                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                    let player_chain_info = db.clone().unwrap().database(&db_name).collection::<schemas::game::InsertPlayerChainToRequest>("player_chain_info"); //-- connecting to player_chain_info collection to insert a new document - we want to deserialize player chain info into the InsertPlayerChainToRequest struct
                                    let player_chain_doc = schemas::game::InsertPlayerChainToRequest{
                                        from_id: update_info.from_id,
                                        to_id: update_info.to_id,
                                        chained_at: Some(now),
                                    };
                                    match player_chain_info.insert_one(player_chain_doc, None).await{
                                        Ok(insert_result) => {
                                            let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                                                data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                                                message: INSERTED,
                                                status: 201,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::OK)
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
    
}








// -------------------------------- get a single player info (during the game) controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------------------------------

pub async fn get_single(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{ //-- this api will return the current status and infos of a player during the game and can be called by the God and the player

     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();
    
    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS || access_level == DEFAULT_USER_ACCESS{ // NOTE - only dev, admin (God) and player can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::game::GetPlayerInfoRequest>(&json){ //-- the generic type of from_str() method is GetPlayerInfoRequest struct - mapping (deserializing) the json string into the GetPlayerInfoRequest struct
                                Ok(player_info) => { //-- we got the username and password inside the login route


                                    ////////////////////////////////// DB Ops

                                    let player_id = ObjectId::parse_str(player_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let users = db.clone().unwrap().database(&db_name).collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch and deserialize all user infos or documents from BSON into the UserInfo struct
                                    match users.find_one(doc! { "_id": player_id }, None).await.unwrap(){
                                        Some(user_doc) => {
                                            let player_info = schemas::game::ReservePlayerInfoResponse{
                                                _id: user_doc._id,
                                                username: user_doc.username,
                                                status: user_doc.status,
                                                role_id: user_doc.role_id,
                                                side_id: user_doc.side_id,
                                            };
                                            let res = Response::builder(); //-- creating a new response cause we didn't find any available route
                                            let response_body = ctx::app::Response::<schemas::game::ReservePlayerInfoResponse>{
                                                message: FETCHED,
                                                data: Some(player_info),
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
    
}















// -------------------------------- get a player role ability info (during the game) controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------------------------------

pub async fn get_player_role_ability(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{ //-- this api will return the current role ability of a specific player during the game and can be called by the God and the player

     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();

    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS || access_level == DEFAULT_USER_ACCESS{ // NOTE - only dev, admin (God) and player can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::game::GetPlayerInfoRequest>(&json){ //-- the generic type of from_str() method is GetPlayerInfoRequest struct - mapping (deserializing) the json string into the GetPlayerInfoRequest struct
                                Ok(player_info) => { //-- we got the username and password inside the login route


                                    ////////////////////////////////// DB Ops

                                    let player_id = ObjectId::parse_str(player_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let player_roles_info = db.clone().unwrap().database(&db_name).collection::<schemas::game::PlayerRoleAbilityInfo>("player_role_ability_info"); //-- connecting to player_role_ability_info collection to update the current_ability field - we want to deserialize all user bsons into the PlayerRoleAbilityInfo struct
                                    match player_roles_info.find_one(doc! { "user_id": player_id }, None).await.unwrap(){
                                        Some(player_role_ability_doc) => {
                                            let response_body = ctx::app::Response::<schemas::game::PlayerRoleAbilityInfo>{
                                                message: FETCHED,
                                                data: Some(player_role_ability_doc),
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
    
}










// -------------------------------- get all player chain infos (during the game) controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------------------------------

pub async fn get_player_chain_infos(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{ //-- this api will return the current chain infos of a specific player during the game and can be called by the God and the player

     

    use routerify::prelude::*;
    let res = Response::builder();
    let db_name = env::var("DB_NAME").expect("⚠️ no db name variable set");
    let db = &req.data::<Option<&Client>>().unwrap().to_owned();

    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS || access_level == DEFAULT_USER_ACCESS{ // NOTE - only dev, admin (God) and player can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::game::GetPlayerInfoRequest>(&json){ //-- the generic type of from_str() method is GetPlayerInfoRequest struct - mapping (deserializing) the json string into the GetPlayerInfoRequest struct
                                Ok(player_info) => { //-- we got the username and password inside the login route


                                    ////////////////////////////////// DB Ops

                                    let player_id = ObjectId::parse_str(player_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let filter = doc! { "from_id": player_id }; //-- filtering all none expired events
                                    let player_chain_info = db.clone().unwrap().database(&db_name).collection::<schemas::game::PlayerChainToInfo>("player_chain_info"); //-- connecting to player_chain_info collection to get a document - we want to deserialize player chain info into the PlayerChainToInfo struct                        
                                    let mut available_chain_infos = schemas::game::AvailableChainInfos{
                                        chain_infos: vec![],
                                    };
                                    match player_chain_info.find(filter, None).await{
                                        Ok(mut cursor) => {
                                            while let Some(event) = cursor.try_next().await.unwrap(){ //-- calling try_next() method on cursor needs the cursor to be mutable - reading while awaiting on try_next() method doesn't return None
                                                available_chain_infos.chain_infos.push(event);
                                            }
                                            let response_body = ctx::app::Response::<schemas::game::AvailableChainInfos>{
                                                message: FETCHED,
                                                data: Some(available_chain_infos),
                                                status: 200,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
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
}