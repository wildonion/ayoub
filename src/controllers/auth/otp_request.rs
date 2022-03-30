





use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use std::env;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{body::HttpBody, Client, header, StatusCode, Body};
use log::info;
use mongodb::bson::doc;
use mongodb::Client as MC;
use rand::prelude::*;
use chrono::Utc;
use std::time::Instant;









// -------------------------------- OTP request controller
//
// -------------------------------------------------------------------------

pub async fn main(db: Option<&MC>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    

    api.post("/auth/otp-req", |req, res| async move{


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::SendOTPRequest>(&json){ //-- the generic type of from_str() method is OTPRequest struct - mapping (deserializing) the json into the OTPRequest struct
                    Ok(otp_req) => { //-- we got the phone number of the user
                        

                        let sms_api_token = env::var("SMS_API_TOKEN").expect("⚠️ no sms api token variable set");
                        let sms_template = env::var("SMS_TEMPLATE").expect("⚠️ no sms template variable set");
                        let phone = otp_req.phone;
                        let code: String = (0..4).map(|_|{
                            let idx = random::<u8>() as usize; //-- idx is one byte cause it's of type u8
                            CHARSET[idx] as char
                        }).collect();
                        
                    
                        

                        let uri = format!("https://api.kavenegar.com/v1/{}/verify/lookup.json?receptor={}&token={}&template={}", sms_api_token, phone, code, sms_template).parse::<hyper::Uri>().unwrap(); //-- parsing it to hyper based uri
                        let client = Client::new();
                        let mut sms_response_streamer = client.get(uri).await.unwrap();


                        
                        while let Some(chunk) = sms_response_streamer.body_mut().data().await{ //-- bodies in hyper are always streamed asynchronously and we have to await for each chunk as it comes in using a while let Some() syntax
                            //-- deserializng from utf8 bytes into the SMSResponse struct to send back as json to user
                            // ...
                        }



                        
                            // benchmark issue
                            // unpacking pointer struct
                            // https://www.reddit.com/r/rust/comments/2s9qzh/how_i_can_generate_random_string_in_rust/ 
                            // https://kavenegar.com/rest.html#sms-Lookup
                            // chiliz
                            // top drive game
                            // multi threading in wasm
                        




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