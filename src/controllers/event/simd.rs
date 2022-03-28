







use std::thread;
use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use crate::utils;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body};
use log::info;
use std::time::Instant;










// -------------------------------- simd controller
//
// -------------------------------------------------------------------------
pub async fn main(api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/event/simd-ops", |req, res| async move{

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::event::Simd>(&json){ //-- the generic type of from_str() method is Simd struct - mapping (deserializing) the json into the Simd struct
                    Ok(simd) => { //-- we got the 32 bits number
                    
                        
                        ////////////////////////////////// SIMD OPS


                        // https://github.com/tokio-rs/tokio/discussions/3858
                        // NOTE - hadnling async task is done using tokio::spawn() method which the task will be solved based on multi threading concept using tokio green threads in the background of the app
                        // NOTE - sharing and mutating clonable data (Arc<Mutex<T>>) between tokio green and rust native threads is done by passing the object through a channel of one of the message passing protocols like mpsc job queue channel


                        //////////////////////////////////
                        ////////////////////////////////// multi threading ops - rust native threads inside the tokio async task 
                        let thread = thread::spawn(|| async move{ //-- the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) for with type either () or a especific type
                        info!("inside the native thread");
                            let async_task = tokio::spawn(async move{ //-- spawning async task to solve it on the background using tokio green threads based on its event loop model - 
                                info!("inside tokio green thread");
                                ////////
                                // ....
                                ////////
                            });
                        });
                        //////////////////////////////////
                        //////////////////////////////////
                        
                        
                        let heavy_func = |chunk: u8| {
                            info!("\t--------Doing some heavy operation on chunk [{:?}]", chunk);
                            chunk
                        };


                        
                        let start = Instant::now();
                        match utils::simd(simd.input, heavy_func).await{
                            Ok(result) => {
                                let end = Instant::now();
                                let delta = end.duration_since(start);
                                let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32; 
                                // assert_eq!(3985935_u32, result); //-- it'll panic on not equal condition
                                info!("::::: the result is {:?} - [it might be different from the input] - | cost : {:?}\n\n", result, delta_ms);
                                let response_body = ctx::app::Response::<u32>{
                                    message: SIMD_RESULT,
                                    data: Some(result),
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
                            },
                            Err(e) => {
                                info!("::::: error in reading chunk caused by {:?}", e);
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