




use crate::contexts as ctx;
use crate::schemas;
use crate::middlewares;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body};
use log::info;
use mongodb::bson::doc;
use mongodb::Client;







// -------------------------------- home controller
//
// -------------------------------------------------------------------------
pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
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