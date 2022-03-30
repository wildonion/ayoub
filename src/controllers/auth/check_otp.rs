






use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;
use chrono::Utc;








// -------------------------------- check OTP controller
//
// -------------------------------------------------------------------------

pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

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
                                } else if time <= otp_info_doc.exp_time{ //-- no need to clone time cause time is of type i64 and it's saved inside the stack
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