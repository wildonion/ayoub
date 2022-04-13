







use crate::schemas;
use crate::contexts as ctx;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::Client;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file
use hyper::http::Uri;





// NOTE - following are in-game api calls which will be called by the God of the game






// -------------------------------- update player role controller
//
// -------------------------------------------------------------------------

pub async fn update_role(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/player/update/role", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
        
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::UserRoleUpdateRequest>(&json){ //-- the generic type of from_str() method is UserRoleUpdateRequest struct - mapping (deserializing) the json into the UserRoleUpdateRequest struct
                    Ok(update_info) => { //-- we got the username and password inside the login route
                        


                    // TODO - get the user_id from the token
                    // ...



                    ////////////////////////////////// DB Ops

                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let role_id = ObjectId::parse_str(update_info.role_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                    let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- connecting to users collection to update the role_id field - we want to deserialize all user bsons into the UserInfo struct
                    match users.find_one_and_update(doc!{"_id": user_id}, doc!{"$set": {"role_id": role_id, "updated_at": Some(now)}}, None).await.unwrap(){
                        Some(user_doc) => {
                            let user_info = schemas::auth::UserUpdateResponse{
                                username: user_doc.username,
                                phone: user_doc.phone,
                                access_level: user_doc.access_level,
                                status: user_doc.status,
                                role_id: user_doc.role_id, // NOTE - updated
                                side_id: user_doc.side_id,
                                created_at: user_doc.created_at,
                                updated_at: Some(now), // NOTE - updated
                                last_login_time: user_doc.last_login_time,
                            };
                            let response_body = ctx::app::Response::<schemas::auth::UserUpdateResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is UserUpdateResponse struct
                                data: Some(user_info),
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










// -------------------------------- update player side controller
//
// -------------------------------------------------------------------------

pub async fn update_side(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/player/update/side", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
       
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::UserSideUpdateRequest>(&json){ //-- the generic type of from_str() method is UserSideUpdateRequest struct - mapping (deserializing) the json into the UserRoleUpdateRequest struct
                    Ok(update_info) => { //-- we got the username and password inside the login route
                        


                    // TODO - get the user_id from the token
                    // ...



                    ////////////////////////////////// DB Ops

                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let side_id = ObjectId::parse_str(update_info.side_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                    let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- connecting to users collection to update the side_id field - we want to deserialize all user bsons into the UserInfo struct
                    match users.find_one_and_update(doc!{"_id": user_id}, doc!{"$set": {"side_id": side_id, "updated_at": Some(now)}}, None).await.unwrap(){
                        Some(user_doc) => {
                            let user_info = schemas::auth::UserUpdateResponse{
                                username: user_doc.username,
                                phone: user_doc.phone,
                                access_level: user_doc.access_level,
                                status: user_doc.status,
                                role_id: user_doc.role_id,
                                side_id: user_doc.side_id, // NOTE - updated
                                created_at: user_doc.created_at,
                                updated_at: Some(now), // NOTE - updated
                                last_login_time: user_doc.last_login_time,
                            };
                            let response_body = ctx::app::Response::<schemas::auth::UserUpdateResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is UserUpdateResponse struct
                                data: Some(user_info),
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







// -------------------------------- update player status controller
//
// -------------------------------------------------------------------------

pub async fn update_status(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/player/update/status", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
       
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::auth::UserStatusUpdateRequest>(&json){ //-- the generic type of from_str() method is UserStatusUpdateRequest struct - mapping (deserializing) the json into the UserRoleUpdateRequest struct
                    Ok(update_info) => { //-- we got the username and password inside the login route
                        


                    // TODO - get the user_id from the token
                    // ...



                    ////////////////////////////////// DB Ops

                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let status = bson::to_bson(&update_info.status).unwrap(); //-- we have to serialize the status to BSON Document object in order to update the mentioned field inside the collection
                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                    let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- connecting to users collection to update the status field - we want to deserialize all user bsons into the UserInfo struct
                    match users.find_one_and_update(doc!{"_id": user_id}, doc!{"$set": {"status": status, "updated_at": Some(now)}}, None).await.unwrap(){
                        Some(user_doc) => {
                            let user_info = schemas::auth::UserUpdateResponse{
                                username: user_doc.username,
                                phone: user_doc.phone,
                                access_level: user_doc.access_level,
                                status: user_doc.status, // NOTE - updated
                                role_id: user_doc.role_id,
                                side_id: user_doc.side_id,
                                created_at: user_doc.created_at,
                                updated_at: Some(now), // NOTE - updated
                                last_login_time: user_doc.last_login_time,
                            };
                            let response_body = ctx::app::Response::<schemas::auth::UserUpdateResponse>{ //-- we have to specify a generic type for data field in Response struct which in our case is UserUpdateResponse struct
                                data: Some(user_info),
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









// -------------------------------- update player role ability controller
//
// -------------------------------------------------------------------------

pub async fn update_role_ability(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/player/update/role-ability", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
       
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::game::UpdatePlayerRoleAbilityRequest>(&json){ //-- the generic type of from_str() method is UpdatePlayerRoleAbilityRequest struct - mapping (deserializing) the json into the UserRoleUpdateRequest struct
                    Ok(update_info) => { //-- we got the username and password inside the login route
                        


                    // TODO - get the user_id from the token
                    // ...



                    ////////////////////////////////// DB Ops

                    let user_id = ObjectId::parse_str(update_info.user_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let event_id = ObjectId::parse_str(update_info.event_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let role_id = ObjectId::parse_str(update_info.role_id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                    let current_ability = bson::to_bson(&update_info.current_ability).unwrap(); //-- we have to serialize the current_ability to BSON Document object in order to update the mentioned field inside the collection
                    let player_roles_info = db.unwrap().database("ayoub").collection::<schemas::game::PlayerRoleAbilityInfo>("player_role_ability_info"); //-- connecting to player_role_ability_info collection to update the current_ability field - we want to deserialize all user bsons into the PlayerRoleAbilityInfo struct
                    match player_roles_info.find_one_and_update(doc!{"user_id": user_id, "event_id": event_id, "role_id": role_id}, doc!{"$set": {"current_ability": current_ability, "updated_at": Some(now)}}, None).await.unwrap(){
                        Some(user_doc) => {
                            let response_body = ctx::app::Response::<schemas::game::PlayerRoleAbilityInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is PlayerRoleAbilityInfo struct
                                data: Some(user_doc),
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











// -------------------------------- chain player to another player controller
//
// ----------------------------------------------------------------------------------

pub async fn chain_to_another_player(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/player/chain", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
       
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::game::InsertPlayerChainToRequest>(&json){ //-- the generic type of from_str() method is InsertPlayerChainToRequest struct - mapping (deserializing) the json into the UserRoleUpdateRequest struct
                    Ok(update_info) => { //-- we got the username and password inside the login route
                        


                    // TODO - get the user_id from the token
                    // ...



                    ////////////////////////////////// DB Ops

                    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                    let player_chain_info = db.unwrap().database("ayoub").collection::<schemas::game::InsertPlayerChainToRequest>("player_chain_info"); //-- connecting to player_chain_info collection to insert a new document - we want to deserialize player chain info into the InsertPlayerChainToRequest struct
                    let player_chain_doc = schemas::game::InsertPlayerChainToRequest{
                        from_id: update_info.from_id,
                        to_id: update_info.to_id,
                        chained_at: Some(now),
                    };
                    match player_chain_info.insert_one(player_chain_doc, None).await{
                        Ok(insert_result) => {
                            let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                                data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                                message: INSERTED,
                                status: 201,
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








// -------------------------------- get a single player info controller
//
// ----------------------------------------------------------------------------------

pub async fn get_single(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/player/get/single", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
       



        // TODO - need admin (God) access level
        // ...


        
        let uri = &req.uri().to_string().parse::<Uri>().unwrap();
        let params = uri.query().unwrap(); //-- extracting all parameters inside the url


        

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::game::GetPlayerRequest>(&json){ //-- the generic type of from_str() method is GetPlayerRequest struct - mapping (deserializing) the json into the GetPlayerRequest struct
                    Ok(player_info) => { //-- we got the username and password inside the login route


                        ////////////////////////////////// DB Ops

                        let player_id = ObjectId::parse_str(player_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                        let users = db.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch and deserialize all user infos or documents from BSON into the UserInfo struct
                        match users.find_one(doc! { "_id": player_id }, None).await.unwrap(){
                            Some(user_doc) => {
                                let player_info = schemas::game::PlayerInfo{
                                    _id: user_doc._id,
                                    username: user_doc.username,
                                    status: user_doc.status,
                                    role_id: user_doc.role_id,
                                    side_id: user_doc.side_id,
                                };
                                let res = Response::builder(); //-- creating a new response cause we didn't find any available route
                                let response_body = ctx::app::Response::<schemas::game::PlayerInfo>{
                                    message: FETCHED,
                                    data: Some(player_info),
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