







use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use mongodb::Client;
use log::info;
use mongodb::bson::{self, oid::ObjectId, doc}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file







// -------------------------------- add role controller
//
// -------------------------------------------------------------------------

pub async fn add(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());

    api.post("/game/role/add", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that


        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::game::AddRoleRequest>(&json){ //-- the generic type of from_str() method is AddRoleRequest struct - mapping (deserializing) the json into the AddRoleRequest struct
                    Ok(role_info) => {


                        let name = role_info.name;
                        let rate = role_info.rate;
                        let desc = role_info.desc;
                        let abilities = role_info.abilities;



                        ////////////////////////////////// DB Ops
                        
                        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec 
                        let roles = db.unwrap().database("ayoub").collection::<schemas::game::AddRoleRequest>("roles"); //-- using AddRoleRequest struct to insert a role info into roles collection 
                        let role_doc = schemas::game::AddRoleRequest{
                            name,
                            rate,
                            desc,
                            abilities,
                            is_disabled: Some(false),
                            created_at: Some(now),
                            updated_at: Some(now),
                        };
                        match roles.insert_one(role_doc, None).await{
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









// -------------------------------- get all roles controller
//
// -------------------------------------------------------------------------
pub async fn all(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    
    api.post("/game/role/get/availables", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that    


        ////////////////////////////////// DB Ops
                        
        let filter = doc! { "is_disabled": false }; //-- filtering all none disabled roles
        let roles = db.unwrap().database("ayoub").collection::<schemas::game::RoleInfo>("roles"); //-- selecting roles collection to fetch and deserialize all roles infos or documents from BSON into the RoleInfo struct
        let mut available_roles = schemas::game::AvailableRoles{
            roles: vec![],
        };

        match roles.find(filter, None).await{
            Ok(mut cursor) => {
                while let Some(role) = cursor.try_next().await.unwrap(){ //-- calling try_next() method on cursor needs the cursor to be mutable - reading while awaiting on try_next() method doesn't return None
                    available_roles.roles.push(role);
                }
                let res = Response::builder(); //-- creating a new response cause we didn't find any available route
                let response_body = ctx::app::Response::<schemas::game::AvailableRoles>{
                    message: FETCHED,
                    data: Some(available_roles),
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
                        .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes here is serialized from the json
                        .unwrap() 
                )
            },
        }

        //////////////////////////////////
        
        
    }).await
}








// -------------------------------- disable role controller
//
// -------------------------------------------------------------------------
pub async fn disable(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    
    api.post("/game/role/diable", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){ //-- read the bytes of the filled buffer with hyper incoming body from the client by calling the reader() method from the Buf trait
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json string
                match serde_json::from_str::<schemas::game::DisableRoleRequest>(&json){ //-- the generic type of from_str() method is DisableRoleRequest struct - mapping (deserializing) the json into the DisableRoleRequest struct
                    Ok(dis_info) => {

                        
                        ////////////////////////////////// DB Ops
                        
                        let role_id = ObjectId::parse_str(dis_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                        let roles = db.unwrap().database("ayoub").collection::<schemas::game::RoleInfo>("roles"); //-- selecting roles collection to fetch all role infos into the RoleInfo struct
                        match roles.find_one_and_update(doc!{"_id": role_id}, doc!{"$set": {"is_disabled": true}}, None).await.unwrap(){ //-- finding role based on role id
                            Some(role_doc) => { //-- deserializing BSON into the RoleInfo struct
                                let response_body = ctx::app::Response::<schemas::game::RoleInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is RoleInfo struct
                                    data: Some(role_doc),
                                    message: UPDATED, //-- collection found in ayoub database
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
                            None => { //-- means we didn't find any document related to this title and we have to tell the user to create a new role
                                let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                    data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                    message: NOT_FOUND_DOCUMENT, //-- document not found in database and the user must do a signup
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