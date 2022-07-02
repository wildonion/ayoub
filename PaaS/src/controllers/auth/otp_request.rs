





use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use crate::utils::gen_random_idx;
use std::{mem, slice, env, io::{BufWriter, Write}};
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use chrono::Duration;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body_bytes and stream buffer
use hyper::{body::HttpBody, Client, header, StatusCode, Body}; //-- HttpBody trait is required to call the data() method on a hyper response body to get the next future IO stream of coming data from the server
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

pub async fn main(db: Option<&MC>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene
    

    api.post("/auth/otp-req", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
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
                        //         GENERATING RANDOM CODE TO SEND IT TO THE RECEPTOR
                        // --------------------------------------------------------------------
                        let code: String = (0..4).map(|_|{
                            let idx = gen_random_idx(random::<u8>() as usize); //-- idx is one byte cause it's of type u8
                            CHARSET[idx] as char //-- CHARSET is of type utf8 bytes thus we can index it 
                        }).collect();
                        let uri = format!("http://api.kavenegar.com/v1/{}/verify/lookup.json?receptor={}&token={}&template={}", sms_api_token, phone, code, sms_template).as_str().parse::<Uri>().unwrap(); //-- parsing it to hyper based uri
                        let client = Client::new();
                        let mut sms_response_stream = client.get(uri).await.unwrap();
                        if sms_response_stream.status() == 200{ /////// the status of the OTP career is 200 means the code has been sent successfully to the receiver
                            







                            // --------------------------------------------------------------------
                            //     COLLECTING ALL INCOMING CHUNKS FROM THE SMS CAREER RESPONSE
                            // --------------------------------------------------------------------
                            let mut buffer = [0u8; SMS_RESPONSE_IO_BUFFER_SIZE]; //-- creating an empty buffer of u8 bytes with the size of the sms response which 286
                            while let Some(next) = sms_response_stream.body_mut().data().await{ //-- bodies in hyper are always streamed asynchronously and we have to await for each chunk as it comes in using a while let Some() syntax
                                let chunk = next?; //-- unwrapping the incoming bytes from the hyper response body inside this iteration  
                                let bytes_as_utf8 = chunk.as_ref(); //-- getting a &[u8] which is in fact a slice of the Bytes by converting or coercing the chunk of type Bytes to &[u8] using as_ref() method
                                buffer.copy_from_slice(bytes_as_utf8); //-- copying all elements inside the bytes_as_utf8 into the buffer, the length of the bytes_as_utf8 must be the same as buffer which in our case the length of the chunk is 286 bytes
                            }








                            // --------------------------------------------------------------------
                            //      DESERIALIZING FROM ut8 BYTES INTO THE SMSResponse STRUCT
                            // --------------------------------------------------------------------
                            match serde_json::from_slice::<schemas::auth::SMSResponse>(&buffer){ //-- we can also use from_reader() method which is slower than from_slice() method and deserialize the bytes of json text directly into the SMSResponse struct - the generic type of from_slice() method is SMSResponse struct - mapping (deserializing) the bytes of json text into the SMSResponse struct
                                Ok(sms_response) => {
                                    




                                        // --------------------------------------------------------------------
                                        //                   SERIALIZING USING serde & borsh
                                        // --------------------------------------------------------------------
                                        let sms_response_serialized_into_bytes: &[u8] = unsafe { slice::from_raw_parts(&sms_response as *const schemas::auth::SMSResponse as *const u8, mem::size_of::<schemas::auth::SMSResponse>()) }; //-- to pass the struct through the socket we have to serialize it into an array of utf8 bytes - from_raw_parts() forms a slice or &[u8] from the pointer and the length and mutually into_raw_parts() returns the raw pointer to the underlying data, the length of the vector (in elements), and the allocated capacity of the data (in elements)
                                        let mut sms_response_serialized_into_vec_bytes_using_serede = serde_json::to_vec(&sms_response).unwrap(); //-- converting the sms_response object into a JSON utf8 byte vector using serde
                                        let mut sms_response_serialized_into_vec_bytes_using_borsh = sms_response.try_to_vec().unwrap(); //-- converting the sms_response object into vector of utf8 bytes using borsh
                                        let deserialize_to_utf8_using_serde_from_slice = serde_json::from_slice::<schemas::auth::SMSResponse>(&sms_response_serialized_into_vec_bytes_using_serede).unwrap(); //-- passing the vector of utf8 bytes into the from_slice() method to deserialize into the SMSResponse struct - since Vec<u8> will be coerced to &'a [u8] at compile time we've passed Vec<u8> into the from_slice() method 
                                        let deserialize_to_utf8_using_borsh_from_slice = schemas::auth::SMSResponse::try_from_slice(&sms_response_serialized_into_vec_bytes_using_borsh).unwrap(); //-- passing the vector of utf8 bytes into the try_from_slice() method to deserialize into the SMSResponse struct - since Vec<u8> will be coerced to &'a [u8] at compile time we've passed Vec<u8> type into the try_from_slice() method
                                        // --------------------------------------------------------------------
                                        //                      CONVERTING Vec<u8> -> &[u8]
                                        // --------------------------------------------------------------------
                                        let mut utf8_bytes_using_as_mut_slice = sms_response_serialized_into_vec_bytes_using_serede.as_mut_slice(); //-- converting Vec<u8> to mutable slice of &[u8] using as_mut_slice() method - remeber that sms_response_serialized_into_vec_bytes_using_serede must be defined as mutable
                                        let utf8_bytes_using_casting: &[u8] = &sms_response_serialized_into_vec_bytes_using_serede; //-- since the Vec<u8> will be coerced to &'a [u8] with a valid lifetime at compile time we can borrow the ownership of sms_response_serialized_into_vec_bytes_using_serede using & which by doing this we're borrowing a slice of Ve<u8> from the heap memory which will be coerced to &'a [u8] since we've specified the type of sms_response_serialized_into_utf8_bytes_using_serede which is &[u8]
                                        let boxed_utf8_bytes_using_box_slcie = sms_response_serialized_into_vec_bytes_using_serede.into_boxed_slice(); //-- converting the Vec<u8> to Box<u8> using into_boxed_slice() method 
                                        let utf_bytes_dereference_from_box = &*boxed_utf8_bytes_using_box_slcie; //-- borrow the ownership of the dereferenced boxed_utf8_bytes_using_box_slcie using & to convert it to &[u8] with a valid lifetime since the dereferenced boxed_utf8_bytes_using_box_slcie has unknown size at compile time thus working with u8 slice needs to borrow them from the heap memory to have their location address due to implemented ?Sized for [u8]







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



                                                // Do what so ever with otp_info object :) 
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
                                                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
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
                                                        let response_body_json = serde_json::to_string(&response_body).unwrap();
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
                                    let response_body_json = serde_json::to_string(&response_body).unwrap();
                                    Ok(
                                        res
                                            .status(StatusCode::NOT_ACCEPTABLE)
                                            .header(header::CONTENT_TYPE, "application/json")
                                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                            .unwrap() 
                                    )
                                },
                            }
                        } else{ /////// the status of the OTP career is not 200 means the code didn't send successfully to the receiver
                            let response_body = ctx::app::Response::<ctx::app::Nill>{
                                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                message: &"OTP didn't send from the career".to_string(), //-- message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                                status: 503,
                            };
                            let response_body_json = serde_json::to_string(&response_body).unwrap();
                            Ok(
                                res
                                    .status(StatusCode::SERVICE_UNAVAILABLE)
                                    .header(header::CONTENT_TYPE, "application/json")
                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
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
                                .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
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
                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                        .unwrap() 
                )
            },
        } 
    }).await
}