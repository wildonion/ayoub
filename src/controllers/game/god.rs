





use crate::schemas;
use crate::contexts as ctx;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::Client;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file







// NOTE - the following api is only for God 










// -------------------------------- create group controller
//
// -------------------------------------------------------------------------

pub async fn create_group(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/god/create/group", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
        

        // TODO - need admin (God) access level
        // TODO - upload image for group prof like tus resumable upload file
        // TODO - first allocate a space on ram in server for file then on every incoming chunk save the file and seek the cursor to that saved chunk on reconnecting to the server
        // TODO - streaming all over the incoming chunks of the file to save them one by one inside a buffer located on the client ram on corruption condition to gather those bytes to form the whole file
        // ...

        
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::game::AddGroupRequest>(&json){ //-- the generic type of from_str() method is AddGroupRequest struct - mapping (deserializing) the json into the AddGroupRequest struct
                    Ok(group_info) => {



                        let group_name = group_info.clone().name; //-- cloning to prevent from moving
                        let group_owner = group_info.clone().owner; //-- the owner id (user id from users collection) of this group - cloning to prevent from moving



                        ////////////////////////////////// DB Ops

                        let groups = db.unwrap().database("ayoub").collection::<schemas::game::GroupInfo>("groups");
                        match groups.find_one(doc!{"group_name": group_info.clone().name}, None).await.unwrap(){
                            Some(group_doc) => { 
                                let response_body = ctx::app::Response::<schemas::game::GroupInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is GroupInfo struct
                                    data: Some(group_doc),
                                    message: FOUND_DOCUMENT,
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
                            None => { //-- no document found with this name thus we must insert a new one into the databse
                                let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                let groups = db.unwrap().database("ayoub").collection::<schemas::game::AddGroupRequest>("groups"); //-- using AddGroupRequest struct to insert a deck info into groups collection 
                                let group_doc = schemas::game::AddGroupRequest{
                                    name: group_name,
                                    owner: group_owner,
                                    created_at: Some(now),
                                    updated_at: Some(now),
                                };
                                match groups.insert_one(group_doc, None).await{
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









// -------------------------------- update group controller
//
// -------------------------------------------------------------------------

pub async fn update_group(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/player/update/role", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
        
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::game::UpdateGroupRequest>(&json){ //-- the generic type of from_str() method is UpdateGroupRequest struct - mapping (deserializing) the json into the UpdateGroupRequest struct
                    Ok(update_info) => { //-- we got the username and password inside the login route
                        

                    
                    // TODO - need admin (God) access level
                    // ...



                    ////////////////////////////////// DB Ops

                    let group_id = ObjectId::parse_str(update_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                    let groups = db.unwrap().database("ayoub").collection::<schemas::game::GroupInfo>("groups"); //-- connecting to groups collection to update the name field - we want to deserialize all user bsons into the GroupInfo struct
                    match groups.find_one_and_update(doc!{"_id": group_id}, doc!{"$set": {"name": update_info.name, "updated_at": Some(now)}}, None).await.unwrap(){
                        Some(group_doc) => {
                            let group_info = schemas::game::GroupInfo{
                                _id: group_doc._id,
                                name: group_doc.name,
                                owner: group_doc.owner,
                                created_at: group_doc.created_at,
                                updated_at: group_doc.updated_at
                            };
                            let response_body = ctx::app::Response::<schemas::game::GroupInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is GroupInfo struct
                                data: Some(group_info),
                                message: UPDATED,
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
                        None => { //-- means we didn't find any document related to this user_id and we have to tell the user do a signup
                            let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                message: NOT_FOUND_PLAYER, //-- document not found in database and the user must do a signup
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