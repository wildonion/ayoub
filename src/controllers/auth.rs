


use crate::contexts as ctx;
use crate::schemas;
use crate::utils;
use crate::middlewares;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;
use chrono::Utc;














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








// -------------------------------- OTP request controller
//
// -------------------------------------------------------------------------

pub async fn opt_request(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/auth/otp-req", |req, res| async move{


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::SendOTPRequest>(&json){ //-- the generic type of from_str() method is OTPRequest struct - mapping (deserializing) the json into the OTPRequest struct
                    Ok(otp_req) => { //-- we got the phone number of the user
                        

                        let phone = otp_req.phone;




                        
                        // 1) generate random code
                        // 2) send generated code to the receptor
                        // 3) on successful status coming from the career upsert the code, phone and 2 mins expiration time into otp_info collection
                        // 4) use SaveOTPInfo struct to insert otp info bson into the mongodb
                        // ... 





                        ////////////////////////////////// DB Ops
                        let otps = db.unwrap().database("ayoub").collection::<schemas::auth::OTPInfo>("otp_info");
                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                            message: NOT_IMPLEMENTED,
                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                            status: 200,
                        };
                        let response_body_json = serde_json::to_string(&response_body).unwrap();
                        Ok(
                            res
                                .status(StatusCode::NOT_IMPLEMENTED) //-- not found route or method not allowed
                                .header(header::CONTENT_TYPE, "application/json")
                                .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
                                .unwrap()
                        )
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









// -------------------------------- check OTP controller
//
// -------------------------------------------------------------------------

pub async fn check_otp(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/auth/check-otp", |req, res| async move{


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::CheckOTPRequest>(&json){ //-- the generic type of from_str() method is CheckOTPRequest struct - mapping (deserializing) the json into the CheckOTPRequest struct
                    Ok(otp_info) => { //-- we got the phone number of the user
                        


                        let code = otp_info.code;
                        let phone = otp_info.phone;
                        let time = otp_info.time;

                        
                        
                        ////////////////////////////////// DB Ops
                        let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users");
                        let otps = db.unwrap().database("ayoub").collection::<schemas::auth::OTPInfo>("otp_info");
                        match otps.find_one(doc!{"phone": phone.clone(), "code": code}, None).await.unwrap(){ // NOTE - we've cloned the phone in order to prevent its ownership from moving
                            Some(otp_info_doc) => {
                                if time > otp_info_doc.exp_time{
                                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                                        message: EXPIRED_OTP_CODE,
                                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                        status: 406,
                                    };
                                    let response_body_json = serde_json::to_string(&response_body).unwrap();
                                    Ok(
                                        res
                                            .status(StatusCode::NOT_ACCEPTABLE) //-- not found route or method not allowed
                                            .header(header::CONTENT_TYPE, "application/json")
                                            .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
                                            .unwrap()
                                    )
                                } else if time <= otp_info_doc.exp_time{
                                    match users.find_one(doc!{"phone": phone.clone()}, None).await.unwrap(){ //-- we're finding the user based on the incoming phone from the clinet - we've cloned the phone in order to prevent its ownership from moving
                                        Some(user_info) => {
                                                match otps.find_one_and_update(doc!{"_id": otp_info_doc._id}, doc!{"$set": {"updated_at": Some(Utc::now().timestamp())}}, None).await.unwrap(){ //-- updating the updated_at field for the current otp_info doc based on the current otp_info doc id 
                                                    Some(updated_otp_info) => {
                                                        let check_otp_response = schemas::auth::CheckOTPResponse{
                                                            user_id: user_info._id, //-- this is of tyoe mongodb bson ObjectId
                                                            otp_info_id: otp_info_doc._id, //-- this is of tyoe mongodb bson ObjectId
                                                            code: otp_info_doc.code,
                                                            phone: otp_info_doc.phone,
                                                            last_otp_login_update: updated_otp_info.updated_at, 
                                                        };
                                                        let response_body = ctx::app::Response::<schemas::auth::CheckOTPResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is CheckOTPResponse struct
                                                            data: Some(check_otp_response), //-- use CheckOTPResponse struct to serialize user info and otp info from bson into the json to send back to the user
                                                            message: ACCESS_DENIED,
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
                                                    None => {
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
                                                    },
                                                }
                                            },
                                        None => {
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
                                        },
                                    }  
                                } else{
                                    todo!();
                                }
                            },
                            None => { //-- means we didn't find any document related to this otp and we have to tell the user do a signup
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








// -------------------------------- home controller
//
// -------------------------------------------------------------------------
pub async fn home(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.get("/auth/home", |req, res| async move{
        
        match middlewares::auth::pass(req).await{
            Ok((token_data, req)) => {
                                
                let _id = token_data.claims._id;
                let username = token_data.claims.username;



                ////////////////////////////////// DB Ops
                        
                let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
                match users.find_one(doc!{"username": username.clone(), "_id": _id.unwrap()}, None).await.unwrap(){ //-- finding user based on username
                    Some(user_doc) => { //-- deserializing BSON into the UserInfo struct
                        let user_response = schemas::auth::CheckTokenResponse{
                            _id: user_doc._id,
                            username: user_doc.username,
                            phone: user_doc.phone,
                            access_level: user_doc.access_level,
                            status: user_doc.status,
                            role_id: user_doc.role_id,
                            side_id: user_doc.side_id,
                            created_at: user_doc.created_at,
                        };
                        let response_body = ctx::app::Response::<schemas::auth::CheckTokenResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is CheckTokenResponse struct
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
            Err(e) => {
                let response_body = ctx::app::Response::<ctx::app::Nill>{
                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                    message: &e, //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
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
    }).await
}











// -------------------------------- check token controller
//
// -------------------------------------------------------------------------
pub async fn check_token(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/auth/check-token", |req, res| async move{

        match middlewares::auth::pass(req).await{
            Ok((token_data, req)) => {
                                
                let _id = token_data.claims._id;
                let username = token_data.claims.username;



                ////////////////////////////////// DB Ops
                        
                let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
                match users.find_one(doc!{"username": username.clone(), "_id": _id.unwrap()}, None).await.unwrap(){ //-- finding user based on username
                    Some(user_doc) => { //-- deserializing BSON into the UserInfo struct
                        let user_response = schemas::auth::CheckTokenResponse{
                            _id: user_doc._id,
                            username: user_doc.username,
                            phone: user_doc.phone,
                            access_level: user_doc.access_level,
                            status: user_doc.status,
                            role_id: user_doc.role_id,
                            side_id: user_doc.side_id,
                            created_at: user_doc.created_at,
                        };
                        let response_body = ctx::app::Response::<schemas::auth::CheckTokenResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is LoginResponse struct
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
            Err(e) => {
                let response_body = ctx::app::Response::<ctx::app::Nill>{
                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                    message: &e, //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
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
    }).await
}









// -------------------------------- login controller
//
// -------------------------------------------------------------------------
pub async fn login(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{


    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/auth/login", |req, res| async move{


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
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
                                        let jwt_payload = utils::jwt::Claims{_id: user_doc.clone()._id, username: user_doc.clone().username, iat: now, exp};
                                        match utils::jwt::construct(jwt_payload).await{
                                            Ok(token) => {
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
                                                };
                                                let response_body = ctx::app::Response::<schemas::auth::LoginResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is LoginResponse struct
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










// -------------------------------- signup controller
//
// -------------------------------------------------------------------------
pub async fn signup(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{


    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/auth/signup", |req, res| async move{    
        
        
        let whole_body_bytes = hyper::body::aggregate(req.into_body()).await.unwrap(); //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all futures stream or chunks which is utf8 bytes - since we don't know the end yet, we can't simply stream the chunks as they arrive (cause all futures stream or chunks which are called chunks are arrived asynchronously), so here we do `.await` on the future, waiting on concatenating the full body after all chunks arrived then afterwards the content can be reversed
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::RegisterRequest>(&json){ //-- the generic type of from_str() method is RegisterRequest struct
                    Ok(user_info) => {





                        ////////////////////////////////// DB Ops
                        
                        let users = db.unwrap().database("ayoub").collection::<schemas::auth::RegisterResponse>("users");
                        match users.find_one(doc!{"username": user_info.clone().username, "phone": user_info.clone().phone}, None).await.unwrap(){ //-- finding user based on username and phone
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
                                let users = db.unwrap().database("ayoub").collection::<schemas::auth::RegisterRequest>("users");
                                match schemas::auth::RegisterRequest::hash_pwd(user_info.pwd).await{
                                    Ok(hash) => {
                                        let user_doc = schemas::auth::RegisterRequest{
                                            username: user_info.username,
                                            phone: user_info.phone,
                                            pwd: hash,
                                            access_level: user_info.access_level,
                                            status: user_info.status,
                                            role_id: None,
                                            side_id: None,
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
                                    }
                                }
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


