







use std::thread;
use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use crate::utils;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer and is used for manipulating coming network bytes from the socket
use hyper::{header, StatusCode, Body};
use log::info;
use std::time::Instant;






/*


        ## About the `/event/simd-ops` route

        [Question](https://quera.org/problemset/113613/)


        ### Inputs

        * An operation function
        * u32 bits number

        ### Output

        * u32 bits number


        ### Sample Input

        * _heavy_func_
        * _3985935_

        ### Sample Output on Equal Condition

        ```console
        INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T18:19:47.883156
        INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T18:19:47.885159800
        INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T18:19:47.885159800
        INFO  simd  >  --------Doing some heavy operation on chunk [0]
        INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T18:19:47.885159800
        INFO  simd  >  --------Doing some heavy operation on chunk [60]
        INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T18:19:47.885159800
        INFO  simd  >  --------Doing some heavy operation on chunk [210]
        INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T18:19:47.886155
        INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T18:19:47.886155
        INFO  simd  >  --------Doing some heavy operation on chunk [15]
        INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T18:19:47.886155
        INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T18:19:47.887157100
        INFO  utils > collected bytes -> [0, 60, 210, 15] at time 2022-03-16T18:19:47.887157100
        INFO  simd  > ::::: the result is 3985935 - [it might be different from the input] - | cost : 4.0779
        ```

        ### Sample Output on Unequal Condition

        ```console
        INFO  utils > chunk 0 in utf8 format -> [0] at time 2022-03-16T18:20:57.775299
        INFO  utils > chunk 1 in utf8 format -> [60] at time 2022-03-16T18:20:57.776326200
        INFO  simd  >  --------Doing some heavy operation on chunk [0]
        INFO  utils > chunk 2 in utf8 format -> [210] at time 2022-03-16T18:20:57.779358200
        INFO  utils > chunk 3 in utf8 format -> [15] at time 2022-03-16T18:20:57.780341
        INFO  utils >  sender-channel---(chunk 0)---receiver-channel at time 2022-03-16T18:20:57.780341
        INFO  simd  >  --------Doing some heavy operation on chunk [60]
        INFO  utils >  sender-channel---(chunk 1)---receiver-channel at time 2022-03-16T18:20:57.783330100
        INFO  utils > collecting all chunks received from the receiver at time 2022-03-16T18:20:57.782328700
        INFO  simd  >  --------Doing some heavy operation on chunk [15]
        INFO  simd  >  --------Doing some heavy operation on chunk [210]
        INFO  utils >  sender-channel---(chunk 3)---receiver-channel at time 2022-03-16T18:20:57.787324900
        INFO  utils >  sender-channel---(chunk 2)---receiver-channel at time 2022-03-16T18:20:57.788324300
        INFO  utils > collected bytes -> [0, 60, 15, 210] at time 2022-03-16T18:20:57.790324800
        INFO  simd  > ::::: the result is 3936210 - [it might be different from the input] - | cost : 15.9839
        ```

        ### The Beauty of Concurrency!

        **NOTE** - Due to the time which takes to send and receive each chunks inside threads through the `mpsc` channel asyncly, the result might be different on each run and it depends on the system, but here at first run both input and the result got into an equality condition.

        
*/






// -------------------------------- simd controller
//
// -------------------------------------------------------------------------
pub async fn main(api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/event/simd-ops", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::event::Simd>(&json){ //-- the generic type of from_str() method is Simd struct - mapping (deserializing) the json into the Simd struct
                    Ok(simd) => { //-- we got the 32 bits number
                    
                        
                        ////////////////////////////////// SIMD OPS


                        // https://github.com/tokio-rs/tokio/discussions/3858
                        // NOTE - mpsc means multiple thread can access the Arc<Mutex<T>> but only one of them can mutate the T; multiple reader but single writer
                        // NOTE - hadnling async task is done using tokio::spawn() method which the task will be solved based on multi threading concept using tokio green threads in the background of the app
                        // NOTE - sharing and mutating clonable data (Arc<Mutex<T>>) between tokio green and rust native threads is done by passing the object through a channel of one of the message passing protocols like mpsc channel
                        // NOTE - to use tokio::spawn(async move{}) and thread::spawn(|| async move{}) we must be inside an async function to await on what's has been spawned on the background



                        //////////////////////////////////
                        ////////////////////////////////// multi threading ops - tokio async task inside the rust native threads 
                        let thread = thread::spawn(move || async move{ //-- the body of the closure is an async block means it'll return a future object (trait Future has implemented for that) with type either () or a especific type
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