






use std::env;
use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use hyper::{header, StatusCode, Body, Response};
use mongodb::{Client, bson, bson::{doc, oid::ObjectId}};
















// -------------------------------- add proposal controller
//
// -------------------------------------------------------------------------
pub async fn add_proposal(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    
    api.post("/proposal/add", |req, res| async move{

        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json
                match serde_json::from_str::<schemas::fishuman::ProposalAddRequest>(&json){ //-- the generic type of from_str() method is ProposalAddRequest struct - mapping (deserializing) the json into the ProposalAddRequest struct
                    Ok(proposal_info) => { //-- we got the username and password inside the login route


                        ////////////////////////////////// DB Ops
                        
                        let proposals = db.unwrap().database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch all proposal infos into the ProposalInfo struct
                        match proposals.find_one(doc!{"title": proposal_info.clone().title}, None).await.unwrap(){ //-- finding proposal based on proposal title
                            Some(proposal_doc) => { //-- deserializing BSON into the ProposalInfo struct
                                let response_body = ctx::app::Response::<schemas::fishuman::ProposalInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is ProposalInfo struct
                                    data: Some(proposal_doc), //-- data is an empty &[u8] array
                                    message: FOUND_DOCUMENT, //-- collection found in fishuman document (database)
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
                            None => { //-- means we didn't find any document related to this title and we have to create a new proposaL
                                let proposals = db.unwrap().database("fishuman").collection::<schemas::fishuman::ProposalAddRequest>("proposals");
                                let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
                                let exp_time = now + env::var("PROPOSAL_EXPIRATION").expect("⚠️ found no proposal expiration time").parse::<i64>().unwrap();
                                let new_proposal = schemas::fishuman::ProposalAddRequest{
                                    title: proposal_info.title,
                                    content: proposal_info.content,
                                    creator_wallet_address: proposal_info.creator_wallet_address,
                                    upvotes: Some(0),
                                    downvotes: Some(0),
                                    voters: Some(vec![]), //-- initializing empty voters
                                    is_expired: Some(false), //-- a proposal is not expired yet or at initialization
                                    expire_at: Some(exp_time), //-- a proposal will be expired at
                                    created_at: Some(now),
                                };
                                match proposals.insert_one(new_proposal, None).await{
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
                                            message: &e.to_string(), //-- take a reference to the string error
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
                            },
                        }

                        //////////////////////////////////


                    },
                    Err(e) => {
                        let response_body = ctx::app::Response::<ctx::app::Nill>{
                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                            message: &e.to_string(), //-- take a reference to the string error
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
                    message: &e.to_string(), //-- take a reference to the string error
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










// -------------------------------- get all proposals controller
//
// -------------------------------------------------------------------------
pub async fn get_all_proposals(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    api.post("/proposal/get/availables", |req, res| async move{
        
        
        ////////////////////////////////// DB Ops
                        
        let filter = doc! { "is_expired": false }; //-- filtering all none expired proposals
        let proposals = db.unwrap().database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch and deserialize all proposal infos or documents from BSON into the ProposalInfo struct
        let mut available_proposals = schemas::fishuman::AvailableProposals{
            proposals: vec![],
        };

        match proposals.find(filter, None).await{
            Ok(mut cursor) => {
                while let Some(proposal) = cursor.try_next().await.unwrap(){ //-- calling try_next() method on cursor needs the cursor to be mutable - reading while awaiting on try_next() method doesn't return None
                    available_proposals.proposals.push(proposal);
                }
                let res = Response::builder(); //-- creating a new response cause we didn't find any available route
                let response_body = ctx::app::Response::<schemas::fishuman::AvailableProposals>{
                    message: FETCHED,
                    data: Some(available_proposals), //-- data is an empty &[u8] array
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
                    message: &e.to_string(), //-- take a reference to the string error
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











// -------------------------------- cast vote proposal controller
//
// -------------------------------------------------------------------------
pub async fn cast_vote_proposal(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{
    
    api.post("/proposal/cast-vote", |req, res| async move{
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json
                match serde_json::from_str::<schemas::fishuman::CastVoteRequest>(&json){ //-- the generic type of from_str() method is CastVoteRequest struct - mapping (deserializing) the json into the CastVoteRequest struct
                    Ok(vote_info) => { //-- we got the username and password inside the login route

                        
                        ////////////////////////////////// DB Ops
                        
                        let proposal_id = ObjectId::parse_str(vote_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string 
                        let proposals = db.unwrap().database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch all proposal infos into the ProposalInfo struct
                        match proposals.find_one(doc!{"_id": proposal_id}, None).await.unwrap(){ //-- finding proposal based on proposal title and id
                            Some(proposal_doc) => { //-- deserializing BSON into the ProposalInfo struct
                                let mut upvotes: u16 = 0; //-- trait Copy is implemented for u16 thus we don't loose the ownership when we move them into a new scope
                                let mut downvotes: u16 = 0; //-- trait Copy is implemented for u16 thus we don't loose the ownership when we move them into a new scope
                                if vote_info.voter.is_upvote{
                                    upvotes+=1;
                                }
                                if !vote_info.voter.is_upvote{
                                    downvotes+=1;
                                }
                                let updated_voters = proposal_doc.clone().add_voter(vote_info.voter).await;
                                let serialized_voters = bson::to_bson(&updated_voters).unwrap(); //-- we have to serialize the updated_voters to BSON Document object in order to update voters field inside the collection
                                let serialized_upvotes = bson::to_bson(&upvotes).unwrap(); //-- we have to serialize the upvotes to BSON Document object in order to update voters field inside the collection
                                let serialized_downvotes = bson::to_bson(&downvotes).unwrap(); //-- we have to serialize the downvotes to BSON Document object in order to update voters field inside the collection
                                match proposals.update_one(doc!{"_id": proposal_id}, doc!{"$set": { "voters": serialized_voters, "upvotes": serialized_upvotes, "downvotes": serialized_downvotes }}, None).await{
                                    Ok(updated_result) => {
                                        let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                                            data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                                            message: UPDATED, //-- collection found in fishuman document (database)
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
                            }, 
                            None => { //-- means we didn't find any document related to this title and we have to tell the user to create a new proposaL
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
                            message: &e.to_string(), //-- take a reference to the string error
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
                    message: &e.to_string(), //-- take a reference to the string error
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











// -------------------------------- expire proposal controller
//
// -------------------------------------------------------------------------
pub async fn expire_proposal(db: Option<&Client>, api: ctx::app::Api) -> Result<hyper::Response<Body>, hyper::Error>{

    api.post("/proposal/set-expire", |req, res| async move{
        let whole_body_bytes = hyper::body::to_bytes(req.into_body()).await?; //-- to read the full body we have to use body::to_bytes or body::aggregate to collect all tcp io stream of future chunk bytes or chunks which is utf8 bytes
        match serde_json::from_reader(whole_body_bytes.reader()){
            Ok(value) => { //-- making a serde value from the buffer which is a future IO stream coming from the client
                let data: serde_json::Value = value;
                let json = serde_json::to_string(&data).unwrap(); //-- converting data into a json
                match serde_json::from_str::<schemas::fishuman::ExpireProposalRequest>(&json){ //-- the generic type of from_str() method is ExpireProposalRequest struct - mapping (deserializing) the json into the ExpireProposalRequest struct
                    Ok(exp_info) => { //-- we got the username and password inside the login route

                        
                        ////////////////////////////////// DB Ops
                        
                        let proposal_id = ObjectId::parse_str(exp_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
                        let proposals = db.unwrap().database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch all proposal infos into the ProposalInfo struct
                        match proposals.find_one_and_update(doc!{"_id": proposal_id}, doc!{"$set": {"is_expired": true}}, None).await.unwrap(){ //-- finding proposal based on proposal id
                            Some(proposal_doc) => { //-- deserializing BSON into the ProposalInfo struct
                                let response_body = ctx::app::Response::<schemas::fishuman::ProposalInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is ProposalInfo struct
                                    data: Some(proposal_doc), //-- data is an empty &[u8] array
                                    message: UPDATED, //-- collection found in fishuman document (database)
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
                            None => { //-- means we didn't find any document related to this title and we have to tell the user to create a new proposaL
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
                            message: &e.to_string(), //-- take a reference to the string error
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
                    message: &e.to_string(), //-- take a reference to the string error
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







// -------------------------------- not found controller
//
// -------------------------------------------------------------------------
pub async fn not_found() -> Result<hyper::Response<Body>, hyper::Error>{
    let res = Response::builder(); //-- creating a new response cause we didn't find any available route
    let response_body = ctx::app::Response::<ctx::app::Nill>{
        message: NOT_FOUND_ROUTE,
        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
        status: 404,
    };
    let response_body_json = serde_json::to_string(&response_body).unwrap();
    Ok(
        res
            .status(StatusCode::NOT_FOUND) //-- not found route or method not allowed
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(response_body_json)) //-- the body of the response must serialized into the utf8 bytes
            .unwrap()
    )
}
