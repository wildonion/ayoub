




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
use std::env;

















// -------------------------------- upload group image controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn upload_img(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", req.uri().path(), chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene

    let res = Response::builder();
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let db = Client::with_uri_str(db_addr).await;
    
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
                            match serde_json::from_str::<schemas::game::AddGroupRequest>(&json){ //-- the generic type of from_str() method is AddGroupRequest struct - mapping (deserializing) the json string into the AddGroupRequest struct
                                Ok(group_info) => {






                                    
                                    // TODO - let filename = utils::upload_asset(path, payload).await; //-- passing the incoming utf8 bytes payload to build the image
                                    // ...
                                    // let res = UploadFile{
                                    //     name: filename,
                                    //     time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                                    // };
                                    // let user = QueryableUser::update_prof_img(id.into_inner(), res).await?;
                                    // Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_UPDATE_SUCCESS, constants::EMPTY)))

                                    





                                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                        message: NOT_IMPLEMENTED,
                                        status: 501,
                                    };
                                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                    Ok(
                                        res
                                            .status(StatusCode::OK)
                                            .header(header::CONTENT_TYPE, "application/json")
                                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                            .unwrap_or(hyper::Response::default()) 
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











// -------------------------------- create group controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn create(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", req.uri().path(), chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene

    let res = Response::builder();
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let db = Client::with_uri_str(db_addr).await;
    
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
                            match serde_json::from_str::<schemas::game::AddGroupRequest>(&json){ //-- the generic type of from_str() method is AddGroupRequest struct - mapping (deserializing) the json string into the AddGroupRequest struct
                                Ok(group_info) => {
                                    


                                    let group_name = group_info.clone().name; //-- cloning to prevent from moving
                                    let group_owner = group_info.clone().owner; //-- the owner id (user id from users collection) of this group - cloning to prevent from moving



                                    ////////////////////////////////// DB Ops

                                    let groups = db.clone().unwrap().database("ayoub").collection::<schemas::game::GroupInfo>("groups");
                                    match groups.find_one(doc!{"group_name": group_info.clone().name}, None).await.unwrap(){
                                        Some(group_doc) => { 
                                            let response_body = ctx::app::Response::<schemas::game::GroupInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is GroupInfo struct
                                                data: Some(group_doc),
                                                message: FOUND_DOCUMENT,
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
                                        None => { //-- no document found with this name thus we must insert a new one into the databse
                                            let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                            let groups = db.clone().unwrap().database("ayoub").collection::<schemas::game::AddGroupRequest>("groups"); //-- using AddGroupRequest struct to insert a deck info into groups collection 
                                            let group_doc = schemas::game::AddGroupRequest{
                                                name: group_name,
                                                owner: group_owner,
                                                created_at: Some(now),
                                                updated_at: Some(now),
                                            };
                                            match groups.insert_one(group_doc, None).await{
                                                Ok(insert_result) => {
                                                    let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                                                        data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                                                        message: REGISTERED,
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
                                                },
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










// -------------------------------- update group controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------

pub async fn update(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", req.uri().path(), chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene

    let res = Response::builder();
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let db = Client::with_uri_str(db_addr).await;
    
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
                            match serde_json::from_str::<schemas::game::UpdateGroupRequest>(&json){ //-- the generic type of from_str() method is UpdateGroupRequest struct - mapping (deserializing) the json string into the UpdateGroupRequest struct
                                Ok(update_info) => { //-- we got the username and password inside the login route
                                

                                    ////////////////////////////////// DB Ops

                                    let group_id = ObjectId::parse_str(update_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                    let groups = db.clone().unwrap().database("ayoub").collection::<schemas::game::GroupInfo>("groups"); //-- connecting to groups collection to update the name field - we want to deserialize all user bsons into the GroupInfo struct
                                    match groups.find_one_and_update(doc!{"_id": group_id}, doc!{"$set": {"name": update_info.name, "updated_at": Some(now)}}, None).await.unwrap(){
                                        Some(group_doc) => {
                                            let group_info = schemas::game::GroupInfo{
                                                _id: group_doc._id,
                                                name: group_doc.name,
                                                owner: group_doc.owner,
                                                created_at: group_doc.created_at,
                                                updated_at: group_doc.updated_at
                                            };
                                            let response_body = ctx::app::Response::<schemas::game::GroupInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is GroupInfo struct
                                                data: Some(group_info),
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












// -------------------------------- get all groups controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------
pub async fn all(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", req.uri().path(), chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene

    let res = Response::builder();
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let db = Client::with_uri_str(db_addr).await;
    
    ////////////////////////////////// DB Ops
                    
    let groups = db.clone().unwrap().database("ayoub").collection::<schemas::game::GroupInfo>("groups"); //-- selecting groups collection to fetch and deserialize all groups infos or documents from BSON into the GroupInfo struct
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
        
}