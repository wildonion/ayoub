


use crate::contexts as ctx;
use crate::schemas;
use crate::utils;
use crate::constants::{ACCESS_GRANTED, ACCESS_DENIED,
                      NOTFOUND_ROUTE, DO_LOGIN, DO_SIGNUP, 
                      WELCOME, UNAUTHORISED, REGISTERED};
use std::thread;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;













// -------------------------------- not found controller
//
// -------------------------------------------------------------------------
pub async fn not_found() -> Result<hyper::Response<Body>, hyper::Error>{
    let res = Response::builder(); //-- creating a new response cause we didn't find any available route
    let response_body = ctx::app::Response::<ctx::app::Nill>{
        message: NOTFOUND_ROUTE,
        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
        status: 404,
    };
    let response_body_json = serde_json::to_string(&response_body).unwrap();
    Ok(
        res
            .status(StatusCode::NOT_FOUND) //-- not found route or method not allowed
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
            .unwrap()
    )
}










// -------------------------------- home controller
//
// -------------------------------------------------------------------------
pub async fn home(api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    api.get("/auth", |req, res| async move{
        let thread = thread::spawn(|| async move{ //-- the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) for with type either () or a especific type
        info!("inside the native thread");
            let async_task = tokio::spawn(async move{ //-- spawning async task to solve it on the background using tokio green threads based on its event loop model - 
                info!("inside tokio green thread");
                ////////
                // ....
                ////////
            });
        });
        let response_body = ctx::app::Response::<ctx::app::Nill>{
            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
            message: WELCOME,
            status: 200,
        };
        let response_body_json = serde_json::to_string(&response_body).unwrap();
        Ok(
            res
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
                .unwrap()
        )
    }).await
}










// -------------------------------- check token controller
//
// -------------------------------------------------------------------------
pub async fn check_token(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    api.post("/auth/check-token", |req, res| async move{


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json
                match serde_json::from_str::<schemas::user::TokenRequest>(&json){ //-- the generic type of from_str() method is LoginRequest struct - mapping (deserializing) the json into the LoginRequest struct
                    Ok(token_request) => { //-- we got the username and password inside the login route

                        
                        match utils::jwt::deconstruct(token_request.access_token.as_str()).await{
                            Ok(decoded_token) => {


                                let _id = decoded_token.claims._id;
                                let username = decoded_token.claims.username;


                                ////////////////////////////////// DB Ops
                        
                                let users = db.unwrap().database("ayoub").collection::<schemas::user::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
                                match users.find_one(doc!{"username": username.clone(), "_id": _id.unwrap()}, None).await.unwrap(){ //-- finding user based on username
                                    Some(user_doc) => { //-- deserializing BSON into the UserInfo struct
                                        let user_response = schemas::user::CheckTokenResponse{
                                            _id: user_doc._id,
                                            username: user_doc.username,
                                            phone: user_doc.phone,
                                            role: user_doc.role,
                                            status: user_doc.status,
                                            created_at: user_doc.created_at,
                                        };
                                        let response_body = ctx::app::Response::<schemas::user::CheckTokenResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is LoginResponse struct
                                            data: Some(user_response), //-- deserialize_from_json_into_struct is of type UserInfo struct 
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
                            Err(e) => { //-- this is the error of can't decode the token
                                let response_body = ctx::app::Response::<ctx::app::Nill>{
                                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                    message: &e.to_string(), //-- take a reference to the string error
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
                            message: &e.to_string(), //-- take a reference to the string error
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
                    message: &e.to_string(), //-- take a reference to the string error
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









// -------------------------------- login controller
//
// -------------------------------------------------------------------------
pub async fn login(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{


    api.post("/auth/login", |req, res| async move{

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json
                match serde_json::from_str::<schemas::user::LoginRequest>(&json){ //-- the generic type of from_str() method is LoginRequest struct - mapping (deserializing) the json into the LoginRequest struct
                    Ok(user_info) => { //-- we got the username and password inside the login route



                        
                        ////////////////////////////////// DB Ops
                        
                        let users = db.unwrap().database("ayoub").collection::<schemas::user::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
                        match users.find_one(doc!{"username": user_info.clone().username}, None).await.unwrap(){ //-- finding user based on username
                            Some(user_doc) => { //-- deserializing BSON into the UserInfo struct
                                match schemas::user::LoginRequest::verify_pwd(user_doc.clone().pwd, user_info.clone().pwd).await{
                                    Ok(_) => { // if we're here means hash and raw are match together and we have the successful login
                                        let (now, exp) = utils::jwt::gen_times().await;
                                        let jwt_payload = utils::jwt::Claims{_id: user_doc.clone()._id, username: user_doc.clone().username, iat: now, exp};
                                        match utils::jwt::construct(jwt_payload).await{
                                            Ok(token) => {
                                                let user_response = schemas::user::LoginResponse{
                                                    _id: user_doc._id,
                                                    access_token: token,
                                                    username: user_doc.username,
                                                    phone: user_doc.phone,
                                                    role: user_doc.role,
                                                    status: user_doc.status,
                                                    created_at: user_doc.created_at,
                                                };
                                                let response_body = ctx::app::Response::<schemas::user::LoginResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is LoginResponse struct
                                                    data: Some(user_response), //-- deserialize_from_json_into_struct is of type UserInfo struct 
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
                                                    message: &e.to_string(), //-- take a reference to the string error
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
                                            message: &e.to_string(), //-- take a reference to the string error
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
                            message: &e.to_string(), //-- take a reference to the string error
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
                    message: &e.to_string(), //-- take a reference to the string error
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









// -------------------------------- signup controller
//
// -------------------------------------------------------------------------
pub async fn signup(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{


    api.post("/auth/signup", |req, res| async move{    
        
        let whole_body_bytes = hyper::body::aggregate(req.into_body()).await.unwrap(); //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all futures stream or chunks which is utf8 bytes - since we don't know the end yet, we can't simply stream the chunks as they arrive (cause all futures stream or chunks which are called chunks are arrived asynchronously), so here we do `.await` on the future, waiting on concatenating the full body after all chunks arrived then afterwards the content can be reversed
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json
                match serde_json::from_str::<schemas::user::RegisterRequest>(&json){ //-- the generic type of from_str() method is RegisterRequest struct
                    Ok(user_info) => {





                        ////////////////////////////////// DB Ops
                        
                        let users = db.unwrap().database("ayoub").collection::<schemas::user::RegisterResponse>("users");
                        match users.find_one(doc!{"username": user_info.clone().username}, None).await.unwrap(){ //-- finding user based on username
                            Some(user_doc) => { //-- if we find a user with this username we have to tell the user do a login 
                                let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                    data: Some(ctx::app::Nill(&[])),
                                    message: DO_LOGIN, //-- please login message
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
                            None => { //-- no document found with this username thus we must insert a new one into the databse
                                let users = db.unwrap().database("ayoub").collection::<schemas::user::RegisterRequest>("users");
                                match schemas::user::RegisterRequest::hash_pwd(user_info.pwd).await{
                                    Ok(hash) => {
                                        let user_doc = schemas::user::RegisterRequest{
                                            username: user_info.username,
                                            phone: user_info.phone,
                                            pwd: hash,
                                            role: user_info.role,
                                            status: user_info.status,
                                            created_at: Some(chrono::Local::now().naive_local()),
                                        };
                                        match users.insert_one(user_doc, None).await{ //-- serializing the user doc which is of type RegisterRequest into the BSON to insert into the mongodb
                                            Ok(insert_result) => {
                                                let response_body = ctx::app::Response::<mongodb::bson::Bson>{ //-- we have to specify a generic type for data field in Response struct which in our case is Bson struct
                                                    data: Some(insert_result.inserted_id),
                                                    message: REGISTERED,
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
                                                    message: &e.to_string(), //-- take a reference to the string error
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
                                    Err(e) => {
                                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                            message: &e.to_string(), //-- take a reference to the string error
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
                                    }
                                }
                            }
                        }

                        //////////////////////////////////




                    },
                    Err(e) => {
                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                            message: &e.to_string(), //-- take a reference to the string error
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
                    message: &e.to_string(), //-- take a reference to the string error
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


