



use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer and is used for manipulating coming network bytes from the socket
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itself cause there is a struct called bson inside the bson.rs file
use mongodb::Client;









// -------------------------------- signup controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------
pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{


    info!("calling {} - {}", api.name, chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene

    api.post("/auth/signup", |req, res| async move{ // NOTE - api will be moved here since neither trait Copy nor Clone is not implemented for that and we can call it only once     
        
        
        let whole_body_bytes = hyper::body::aggregate(req.into_body()).await.unwrap(); //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all futures stream or chunks which is utf8 bytes - since we don't know the end yet, we can't simply stream the chunks as they arrive (cause all futures stream or chunks which are called chunks are arrived asynchronously), so here we do `.await` on the future, waiting on concatenating the full body after all chunks arrived then afterwards the content can be reversed
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
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
                                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                        .unwrap() 
                                )        
                            }, 
                            None => { //-- no document found with this username thus we must insert a new one into the databse
                                let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                let users = db.unwrap().database("ayoub").collection::<schemas::auth::RegisterRequest>("users");
                                match schemas::auth::RegisterRequest::hash_pwd(user_info.pwd).await{
                                    Ok(hash) => {
                                        let user_doc = schemas::auth::RegisterRequest{
                                            username: user_info.username,
                                            phone: user_info.phone,
                                            pwd: hash,
                                            access_level: Some(DEFAULT_USER_ACCESS), //-- default access is the user access
                                            status: DEFAULT_STATUS, //-- setting the user (player) status to default which is 0
                                            role_id: None,
                                            side_id: None,
                                            created_at: Some(now),
                                            updated_at: Some(now),
                                            last_login_time: Some(now),
                                        };
                                        match users.insert_one(user_doc, None).await{ //-- serializing the user doc which is of type RegisterRequest into the BSON to insert into the mongodb
                                            Ok(insert_result) => {
                                                let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                                                    data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                                                    message: REGISTERED,
                                                    status: 200,
                                                };
                                                let response_body_json = serde_json::to_string(&response_body).unwrap();
                                                Ok(
                                                    res
                                                        .status(StatusCode::OK)
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
                                                .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
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