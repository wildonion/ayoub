





use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use crate::utils::gen_random_idx;
use std::{env, io::{BufWriter, Write}};
use chrono::Duration;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body_bytes and stream buffer
use hyper::{body::HttpBody, Client, header, StatusCode, Body};
use log::info;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file
use mongodb::Client as MC;
use rand::prelude::*;
use chrono::prelude::*;
use hyper::http::Uri;
use serde::{Serialize, Deserialize}; //-- to use the deserialize() and serialize() methods on struct we must use these traits










// -------------------------------- OTP request controller
//
// -------------------------------------------------------------------------

pub async fn main(db: Option<&MC>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    

    api.post("/auth/otp-req", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::SendOTPRequest>(&json){ //-- the generic type of from_str() method is OTPRequest struct - mapping (deserializing) the json into the OTPRequest struct
                    Ok(otp_req) => { //-- we got the phone number of the user
                        


                        let phone = otp_req.phone;
                        let sms_api_token = env::var("SMS_API_TOKEN").expect("⚠️ no sms api token variable set");
                        let sms_template = env::var("SMS_TEMPLATE").expect("⚠️ no sms template variable set");
                        
                        

                        

                        
                        // --------------------------------------------------------------------
                        //          GENERATING RANDOM CODE TO SEND IT TO THE RECEPTOR
                        // --------------------------------------------------------------------
                        let code: String = (0..4).map(|_|{
                            let idx = gen_random_idx(random::<u8>() as usize); //-- idx is one byte cause it's of type u8
                            CHARSET[idx] as char //-- CHARSET is of type utf8 bytes thus we can slice it 
                        }).collect();
                        let uri = format!("http://api.kavenegar.com/v1/{}/verify/lookup.json?receptor={}&token={}&template={}", sms_api_token, phone, code, sms_template).as_str().parse::<Uri>().unwrap(); //-- parsing it to hyper based uri
                        let client = Client::new();
                        let mut sms_response_streamer = client.get(uri).await.unwrap();
                        
                        




                        // --------------------------------------------------------------------
                        //     COLLECTING ALL INCOMING CHUNKS FROM THE SMS CAREER RESPONSE
                        // --------------------------------------------------------------------
                        let mut buffer = [0u8; IO_BUFFER_SIZE];
                        let mut stream = BufWriter::new(buffer.as_mut()); //-- creating a new buffer writer from the defined buffer mutably
                        while let Some(next) = sms_response_streamer.body_mut().data().await{ //-- bodies in hyper are always streamed asynchronously and we have to await for each chunk as it comes in using a while let Some() syntax
                            let chunk = next?;
                            write!(stream, "{:#?}", chunk).unwrap(); //-- collecting each incoming chunks to write them into the defined stream buffer to deserialize from utf8 bytes into the SMSResponse struct to send back as json to user
                        }


   


                        
                        // --------------------------------------------------------------------
                        //       DESERIALIZING FROM ut8 BYTES INTO THE SMSResponse STRUCT
                        // --------------------------------------------------------------------
                        match serde_json::from_reader(stream.buffer().reader()){ //-- buffer() method returns a reference to the internally buffered data (means &[u8]) then we can read the bytes of the buffer by calling the reader() method from the Buf trait
                            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                                let data: serde_json::Value = value;
                                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                                match serde_json::from_str::<schemas::auth::SMSResponse>(&json){ //-- the generic type of from_str() method is SMSResponse struct - mapping (deserializing) the json into the SMSResponse struct
                                    Ok(sms_response) => {



                                        if sms_response.r#return.status == 200{ //-- means the code has been sent to telecommunications
                                            
                                            

                                            // --------------------------------------------------------------------
                                            //          GENERATING TWO MINS LATER EXPIRATION TIME FROM NOW
                                            // --------------------------------------------------------------------
                                            let now = Local::now();
                                            let two_mins_later = (now + Duration::seconds(120)).naive_local().timestamp(); //-- generating a timestamp from now till the two mins later
    
                                            
    
    
                                            ////////////////////////////////// DB Ops
                                            
                                            let updated_at = Some(now.timestamp());
                                            let serialized_updated_at = bson::to_bson(&updated_at).unwrap(); //-- we have to serialize the updated_at to BSON Document object in order to update the mentioned field inside the collection
                                            let otp_info = db.unwrap().database("ayoub").collection::<schemas::auth::OTPInfo>("otp_info"); //-- using OTPInfo struct to find and update an otp info inside the otp_info collection
                                            match otp_info.find_one_and_update(doc!{"phone": phone.clone()}, doc!{"$set": {"code": code.clone(), "exp_time": two_mins_later, "updated_at": updated_at}}, None).await.unwrap(){ //-- updated_at is of type i64 thus we don't need to serialize it to bson in order to insert into the collection
                                                Some(otp_info) => { //-- once we get here means that the user is already exists in the collection and we have to save the generated new otp code along with a new expiration time for him/her
    
    
                                                    // ---
                                                    // ...
    
    
                                                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                                                        message: OTP_CODE_HAS_BEEN_SENT,
                                                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                                        status: 200,
                                                    };
                                                    let response_body_json = serde_json::to_string(&response_body).unwrap();
                                                    Ok(
                                                        res
                                                            .status(StatusCode::OK) //-- not found route or method not allowed
                                                            .header(header::CONTENT_TYPE, "application/json")
                                                            .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
                                                            .unwrap()
                                                    )
                                                },
                                                None => { //-- once we get here means that the user is trying to login for the first time in our app and we have to save a new otp info into our otp_info collection
                                                    let otp_info = db.unwrap().database("ayoub").collection::<schemas::auth::SaveOTPInfo>("otp_info"); //-- using SaveOTPInfo struct to insert new otp info into the otp_info collection
                                                    let now = Local::now();
                                                    let new_otp_info = schemas::auth::SaveOTPInfo{
                                                        exp_time: two_mins_later,
                                                        code, //-- no need to clone the code cause we won't use it inside other scope and this is the final place when we use it
                                                        phone, //-- no need to clone the phone cause we won't use it inside other scope and this is the final place when we use it
                                                        created_at: Some(now.timestamp()),
                                                        updated_at: Some(now.timestamp()),
                                                    };
                                                    match otp_info.insert_one(new_otp_info, None).await{
                                                        Ok(insert_result) => {
                                                            let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                                                                data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                                                                message: INSERTED,
                                                                status: 201,
                                                            };
                                                            let response_body_json = serde_json::to_string(&response_body).unwrap();
                                                            Ok(
                                                                res
                                                                    .status(StatusCode::CREATED)
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
                                                        },
                                                    }
                                                },
                                            }
                                            
                                            ////////////////////////////////// 

                                            

                                        } else{
                                            let response_body = ctx::app::Response::<ctx::app::Nill>{
                                                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                                message: &sms_response.r#return.message, //-- converting String to &str by taking a reference to the String location inside the heap cause String will be coerced into &str at compile time
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