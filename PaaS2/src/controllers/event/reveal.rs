






use crate::middlewares;
use crate::utils;
use crate::schemas;
use crate::contexts as ctx;
use crate::constants::*;
use chrono::Utc;
use futures::StreamExt;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer and is used for manipulating coming network bytes from the socket
use hyper::{header, StatusCode, Body, Response, Request};
use log::info;
use mongodb::Client;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itself cause there is a struct called bson inside the bson.rs file
use std::env;
















// -------------------------------- process payment controller
// ➝ Return : Hyper Response Body or Hyper Error
// -------------------------------------------------------------------------
pub async fn role(req: Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{

    
    info!("calling {} - {}", req.uri().path(), chrono::Local::now().naive_local()); //-- info!() macro will borrow the api and add & behind the scene

    let res = Response::builder();
    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let db = Client::with_uri_str(db_addr).await;

    match middlewares::auth::pass(req).await{
        Ok((token_data, req)) => { //-- the decoded token and the request object will be returned from the function call since the Copy and Clone trait is not implemented for the hyper Request and Response object thus we can't have borrow the req object by passing it into the pass() function therefore it'll be moved and we have to return it from the pass() function   
                            
            
    
            let _id = token_data.claims._id;
            let username = token_data.claims.username;
            let access_level = token_data.claims.access_level;
    
            
            let db_to_pass = db.as_ref().unwrap().clone();
            if middlewares::auth::user::exists(Some(&db_to_pass), _id, username, access_level).await{ //-- finding the user with these info extracted from jwt
                if access_level == ADMIN_ACCESS || access_level == DEV_ACCESS{ // NOTE - only dev and admin (God) can handle this route
                    let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp IO stream of future chunk bytes or chunks which is of type utf8 bytes to concatenate the buffers from a body into a single Bytes asynchronously
                    match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
                        Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                            let data: serde_json::Value = value;
                            let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                            match serde_json::from_str::<schemas::event::GetEventRequest>(&json){ //-- the generic type of from_str() method is GetEventRequest struct - mapping (deserializing) the json string into the GetEventRequest struct
                                Ok(event_info) => {

                                    
                                    ////////////////////////////////// DB Ops
                                    
                                    let mut updated_players = vec![]; //-- vector of all updated players
                                    let player_roles_info = db.clone().unwrap().database("ayoub").collection::<schemas::game::InsertPlayerRoleAbilityRequest>("player_role_ability_info"); //-- connecting to player_role_ability_info collection to insert the current_ability field - we want to deserialize all player role ability infos into the InsertPlayerRoleAbilityRequest struct
                                    let roles = db.clone().unwrap().database("ayoub").collection::<schemas::game::RoleInfo>("roles");
                                    let users = db.clone().unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting events collection to fetch and deserialize all user infos or documents from BSON into the UserInfo struct
                                    let event_id = ObjectId::parse_str(event_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                                    let events = db.clone().unwrap().database("ayoub").collection::<schemas::event::EventInfo>("events"); //-- selecting events collection to fetch and deserialize all event infos or documents from BSON into the EventInfo struct
                                    match events.find_one(doc! { "_id": event_id, "is_expired": false }, None).await.unwrap(){ //-- getting a none expired event
                                        Some(event_doc) => {

                                            // ------------------------------------------
                                            for mut p in event_doc.clone().players.unwrap(){ //-- p must be mutable since we want to mutate role_id and side_id fields - we must clone the event_doc in each iteration in order not to lose its ownership during the iteration process
                                                
                                                let random_role_id: ObjectId;
                                                let random_side_id: ObjectId;
                                                let mut unique_random_roles: Vec<schemas::game::RoleInfo> = Vec::new();
                                                
                                                // ------------------------------ FETCHING RANDOM DOCUMENT FROM ROLES COLLECTION ------------------------------
                                                // 
                                                // ------------------------------------------------------------------------------------------------------------
                                                let random_record_setup = doc!{"$sample": {"size": 1}};
                                                let pipeline = vec![random_record_setup];
                                                let random_role: Vec<schemas::game::RoleInfo> = match roles.aggregate(pipeline, None).await{
                                                    Ok(mut cursor) => {
                                                        while let Some(random_doc) = cursor.try_next().await.unwrap(){
                                                            let random_role_info = bson::from_document::<schemas::game::RoleInfo>(random_doc).unwrap();
                                                            if !unique_random_roles.contains(&random_role_info){ //-- the fethced role must be unique since every player must have a unique role :))
                                                                unique_random_roles.push(random_role_info)
                                                            }
                                                        }
                                                        unique_random_roles
                                                    },
                                                    Err(e) => vec![],
                                                };

                                                // ------------------------------ UPDATING PLAYER ROLE ID ------------------------------
                                                // 
                                                // -------------------------------------------------------------------------------------
                                                random_role_id = random_role.iter().take(1).next().unwrap()._id.unwrap();
                                                random_side_id = random_role.iter().take(1).next().unwrap().side_id.unwrap();
                                                p.role_id = Some(random_role_id); //-- updating the role_id field of a player inside the players field of an event 
                                                p.side_id = Some(random_side_id); //-- updating the side_id field of a player inside the players field of an event
                                                let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                                
                                                // ------------------------------ UPDATING USERS COLLECTION ------------------------------
                                                // 
                                                // ---------------------------------------------------------------------------------------
                                                match users.find_one_and_update(doc! { "_id": p._id }, doc!{"$set": {"role_id": random_role_id, "side_id": random_side_id, "updated_at": Some(now)}}, None).await.unwrap(){ //-- finding user based on _id
                                                    Some(user_doc) => { //-- we updated the users collection successfully now we have to update each player inside the current event  
                                                        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                                                        let player_role_ability_info = schemas::game::InsertPlayerRoleAbilityRequest{
                                                            user_id: user_doc._id.unwrap().to_string(), //-- converting the Option<ObjectId> to ObjectId then into String
                                                            role_id: random_role_id.to_string(), //-- converting the ObjectId into String
                                                            even_id: event_doc._id.unwrap().to_string(), //-- converting the Option<ObjectId> to ObjectId then into String
                                                            current_ability: None, //-- initialized None on inserting new doc
                                                            created_at: Some(now),
                                                            updated_at: Some(now),
                                                        };
    
                                                        // ------------------------------ INSERT PLAYER ROLE ABILITY INFO ------------------------------
                                                        // ---------------------------------------------------------------------------------------------
                                                        match player_roles_info.insert_one(player_role_ability_info, None).await{
                                                            Ok(insert_result) => { println!("new player role ability insert successfully at time {} with _id {:?}", chrono::Local::now().naive_local(), insert_result); },
                                                            Err(e) => { println!("error in inserting new player role ability at time {} - {}", chrono::Local::now().naive_local(), e); },
                                                        }        
                                                    },
                                                    None => {}, //-- TODO - means we didn't find any document related to this user_id and simply we return the unmatched player info
                                                }                                                
                                                
                                                updated_players.push(p);

                                            } // ------------------------------------------ end of iterating over event players
                                        
                                            
                                            
                                            // ------------------------------ UPDATING PLAYERS FIELD IN EVENTS COLLECTION ------------------------------
                                            // 
                                            // ---------------------------------------------------------------------------------------------------------
                                            println!(">>>>>>>>>>>>>>>> {:?}", updated_players);
                                            let updated_player_roles = updated_players; //-- getting the updated players
                                            let serialized_updated_player_roles = bson::to_bson(&updated_player_roles).unwrap(); //-- serializing the players field into the BSON to insert into the events collection
                                            let updated_event = match events.find_one_and_update(doc!{"_id": event_doc._id}, doc!{"$set": {"players": serialized_updated_player_roles}}, None).await.unwrap(){ //-- finding event based on event id
                                                Some(event_doc) => Some(event_doc), //-- deserializing BSON (the record type fetched from mongodb) into the EventInfo struct
                                                None => None, //-- means we didn't find any document related to this title and we have to tell the user to create a new event                                                        
                                            };
                                            let response_body = ctx::app::Response::<schemas::event::EventInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is EventInfo struct
                                                data: Some(updated_event.unwrap()),
                                                message: UPDATED, //-- collection found in ayoub database
                                                status: 200,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::OK)
                                                    .header(header::CONTENT_TYPE, "application/json")
                                                    .header("Access-Control-Allow-Origin", "*")
                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                                    .unwrap() 
                                            )
                                        },
                                        None => { //-- means we didn't find any document related to this user_id and we have to tell the user do a signup
                                            let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                                message: NOT_FOUND_DOCUMENT, //-- document not found in database and the user must do a signup
                                                status: 404,
                                            };
                                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                            Ok(
                                                res
                                                    .status(StatusCode::NOT_FOUND)
                                                    .header(header::CONTENT_TYPE, "application/json")
                                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
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
                                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                                    Ok(
                                        res
                                            .status(StatusCode::NOT_ACCEPTABLE)
                                            .header(header::CONTENT_TYPE, "application/json")
                                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                            .unwrap_or(hyper::Response::default()) 
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
                            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                            Ok(
                                res
                                    .status(StatusCode::BAD_REQUEST)
                                    .header(header::CONTENT_TYPE, "application/json")
                                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                                    .unwrap() 
                            )
                        },
                    }
                } else{ //-- access denied for this user with none admin and dev access level
                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                        message: ACCESS_DENIED,
                        status: 403,
                    };
                    let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                    Ok(
                        res
                            .status(StatusCode::BAD_REQUEST)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                            .unwrap() 
                    )
                }
            } else{ //-- user doesn't exist :(
                let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                    message: DO_SIGNUP, //-- document not found in database and the user must do a signup
                    status: 404,
                };
                let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
                Ok(
                    res
                        .status(StatusCode::NOT_FOUND)
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                        .unwrap() 
                )
            }
        },
        Err(e) => {
            let response_body = ctx::app::Response::<ctx::app::Nill>{
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                message: &e, //-- e is of type String and message must be of type &str thus by taking a reference to the String we can convert or coerce it to &str
                status: 500,
            };
            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
            Ok(
                res
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket here is serialized from the json
                    .unwrap() 
            )
        },
    }

}