





use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;
use chrono::Utc;
use rand::prelude::*;









// -------------------------------- OTP request controller
//
// -------------------------------------------------------------------------

pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

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
                        let mut rng = thread_rng();
                        let code: String = (0..4).map(|_|{
                            let idx = rng.gen_range(0..CHARSET.len());
                            CHARSET[idx] as char
                        }).collect();



                        
   
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