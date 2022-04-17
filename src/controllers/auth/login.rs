




use crate::contexts as ctx;
use crate::schemas;
use crate::utils;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;








// -------------------------------- login controller
//
// -------------------------------------------------------------------------
pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{


    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/auth/login", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::LoginRequest>(&json){ //-- the generic type of from_str() method is LoginRequest struct - mapping (deserializing) the json into the LoginRequest struct
                    Ok(user_info) => { //-- we got the username and password inside the login route



                        
                        ////////////////////////////////// DB Ops
                        
                        let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
                        match users.find_one(doc!{"username": user_info.clone().username}, None).await.unwrap(){ //-- finding user based on username
                            Some(user_doc) => { //-- deserializing BSON into the UserInfo struct
                                match schemas::auth::LoginRequest::verify_pwd(user_doc.clone().pwd, user_info.clone().pwd).await{
                                    Ok(_) => { // if we're here means hash and raw are match together and we have the successful login
                                        let (now, exp) = utils::jwt::gen_times().await;
                                        let jwt_payload = utils::jwt::Claims{_id: user_doc.clone()._id, username: user_doc.clone().username, access_level: user_doc.access_level, iat: now, exp}; //-- building jwt if passwords are matched
                                        match utils::jwt::construct(jwt_payload).await{
                                            Ok(token) => {
                                                let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
                                                let user_response = schemas::auth::LoginResponse{
                                                    _id: user_doc._id,
                                                    access_token: token,
                                                    username: user_doc.username,
                                                    phone: user_doc.phone,
                                                    access_level: user_doc.access_level,
                                                    status: user_doc.status,
                                                    role_id: user_doc.role_id,
                                                    side_id: user_doc.side_id,
                                                    created_at: user_doc.created_at,
                                                    updated_at: user_doc.updated_at,
                                                    last_login_time: Some(now),
                                                };
                                                let response_body = ctx::app::Response::<schemas::auth::LoginResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is LoginResponse struct
                                                    data: Some(user_response),
                                                    message: ACCESS_GRANTED,
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
                            }, 
                            None => { //-- means we didn't find any document related to this username and we have to tell the user do a signup
                                let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                    message: DO_SIGNUP, //-- document not found in database and the user must do a signup
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
