




use std::env;
use std::sync::Arc;
use crate::contexts as ctx;
use crate::schemas;
use crate::constants::*;
use chrono::Utc;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- based on orphan rule TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream
use bytes::Buf; //-- based on orphan rule it'll be needed to call the reader() method on the whole_body buffer
use mongodb::{sync::Client, bson, bson::{doc, oid::ObjectId}}; //-- we're using sync mongodb cause mongodb requires tokio to be in Cargo.toml and there is a confliction with the actix tokio
use actix_web::{Error, HttpRequest, HttpResponse, Result, get, post, web};











#[post("/add")]
async fn add_proposal(req: HttpRequest, proposal_info: web::Json<schemas::fishuman::ProposalAddRequest>) -> Result<HttpResponse, Error>{
    
    // let storage = req.app_data::<web::Data<Option<Arc<ctx::app::Storage>>>>().unwrap(); //-- unwrapping the db inside the web data structure which is passed inside the app_data() method
    // let app_storage = match storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().mode{ //-- here as_ref() method convert &Option<T> to Option<&T>
    //     ctx::app::Mode::On => storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
    //     ctx::app::Mode::Off => None, //-- no db is available cause it's off
    // };

    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let app_storage = Client::with_uri_str(db_addr).unwrap();

    let proposal_info = proposal_info.into_inner();    
    let proposals = app_storage.database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch all proposal infos into the ProposalInfo struct
    match proposals.find_one(doc!{"title": proposal_info.clone().title}, None).unwrap(){ //-- finding proposal based on proposal title
        Some(proposal_doc) => { //-- deserializing BSON into the ProposalInfo struct
            let response_body = ctx::app::Response::<schemas::fishuman::ProposalInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is ProposalInfo struct
                data: Some(proposal_doc), //-- data is an empty &[u8] array
                message: FOUND_DOCUMENT, //-- collection found in fishuman document (database)
                status: 302,
            };
            Ok(
                HttpResponse::Found()
                .json(
                    response_body
                ).into_body() //-- call this method in order not to get failed to fetch in client side
            )
        }, 
        None => { //-- means we didn't find any document related to this title and we have to create a new proposaL
            let proposals = app_storage.database("fishuman").collection::<schemas::fishuman::ProposalAddRequest>("proposals");
            let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nano to sec
            let exp_time = now + env::var("PROPOSAL_EXPIRATION").expect("⚠️ found no proposal expiration time").parse::<i64>().unwrap();
            let new_proposal = schemas::fishuman::ProposalAddRequest{
                title: proposal_info.clone().title,
                content: proposal_info.clone().content,
                creator_wallet_address: proposal_info.clone().creator_wallet_address,
                upvotes: Some(0),
                downvotes: Some(0),
                voters: Some(vec![]), //-- initializing empty voters
                is_expired: Some(false), //-- a proposal is not expired yet or at initialization
                expire_at: Some(exp_time), //-- a proposal will be expired at
                created_at: Some(now),
            };
            match proposals.insert_one(new_proposal, None){
                Ok(insert_result) => {
                    let response_body = ctx::app::Response::<ObjectId>{ //-- we have to specify a generic type for data field in Response struct which in our case is ObjectId struct
                        data: Some(insert_result.inserted_id.as_object_id().unwrap()),
                        message: INSERTED,
                        status: 201,
                    };
                    Ok(
                        HttpResponse::Created().json(
                            response_body
                        ).into_body() //-- call this method in order not to get failed to fetch in client side
                    )
                },
                Err(e) => {
                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                        message: &e.to_string(), //-- take a reference to the string error
                        status: 406,
                    };
                    Ok(
                        HttpResponse::NotAcceptable().json(
                            response_body
                        ).into_body() //-- call this method in order not to get failed to fetch in client side
                    )
                }
            }
        },
    }

}


#[get("/get/availables")]
async fn get_all_proposals(req: HttpRequest) -> Result<HttpResponse, Error>{

    // let storage = req.app_data::<web::Data<Option<Arc<ctx::app::Storage>>>>().unwrap(); //-- unwrapping the db inside the web data structure which is passed inside the app_data() method
    // let app_storage = match storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().mode{ //-- here as_ref() method convert &Option<T> to Option<&T>
    //     ctx::app::Mode::On => storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
    //     ctx::app::Mode::Off => None, //-- no db is available cause it's off
    // };

    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let app_storage = Client::with_uri_str(db_addr).unwrap();

    let filter = doc! { "is_expired": false }; //-- filtering all none expired proposals
    let proposals = app_storage.database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch and deserialize all proposal infos or documents from BSON into the ProposalInfo struct
    let mut available_proposals = schemas::fishuman::AvailableProposals{
        proposals: vec![],
    };

    match proposals.find(filter, None){
        Ok(cursor) => {
            // NOTE - uncomment this for async mongodb
            // while let Some(proposal) = cursor.try_next().await.unwrap(){ //-- calling try_next() method on cursor needs the cursor to be mutable - reading while awaiting on try_next() method doesn't return None
            //     available_proposals.proposals.push(proposal);
            // }
            for proposal in cursor {
                available_proposals.proposals.push(proposal.unwrap());
            }
            let response_body = ctx::app::Response::<schemas::fishuman::AvailableProposals>{
                message: FETCHED,
                data: Some(available_proposals), //-- data is an empty &[u8] array
                status: 200,
            };
            Ok(
                HttpResponse::Ok().json(
                    response_body
                ).into_body() //-- call this method in order not to get failed to fetch in client side
            )
        },
        Err(e) => {
            let response_body = ctx::app::Response::<ctx::app::Nill>{
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                message: &e.to_string(), //-- take a reference to the string error
                status: 500,
            };
            Ok(
                HttpResponse::InternalServerError().json(
                    response_body
                ).into_body() //-- call this method in order not to get failed to fetch in client side
            )
        },
    }
    
}


#[post("/cast-vote")]
async fn cast_vote_proposal(req: HttpRequest, vote_info: web::Json<schemas::fishuman::CastVoteRequest>) -> Result<HttpResponse, Error>{
    
    // let storage = req.app_data::<web::Data<Option<Arc<ctx::app::Storage>>>>().unwrap(); //-- unwrapping the db inside the web data structure which is passed inside the app_data() method //-- unwrapping the db inside the web data structure which is passed inside the app_data() method
    // let app_storage = match storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().mode{ //-- here as_ref() method convert &Option<T> to Option<&T>
    //     ctx::app::Mode::On => storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
    //     ctx::app::Mode::Off => None, //-- no db is available cause it's off
    // };

    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let app_storage = Client::with_uri_str(db_addr).unwrap();

    let vote_info = vote_info.into_inner();
    let proposal_id = ObjectId::parse_str(vote_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string 
    let proposals = app_storage.database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch all proposal infos into the ProposalInfo struct
    match proposals.find_one(doc!{"_id": proposal_id}, None).unwrap(){ //-- finding proposal based on proposal title and id
        Some(proposal_doc) => { //-- deserializing BSON into the ProposalInfo struct
            let mut upvotes = proposal_doc.upvotes.unwrap(); //-- trait Copy is implemented for u16 thus we don't loose the ownership when we move them into a new scope
            let mut downvotes = proposal_doc.downvotes.unwrap(); //-- trait Copy is implemented for u16 thus we don't loose the ownership when we move them into a new scope
            if vote_info.voter.is_upvote{
                upvotes+=1;
            }
            if !vote_info.voter.is_upvote{
                downvotes+=1;
            }
            let updated_voters = proposal_doc.clone().add_voter(vote_info.clone().voter).await;
            let serialized_voters = bson::to_bson(&updated_voters).unwrap(); //-- we have to serialize the updated_voters to BSON Document object in order to update voters field inside the collection
            let serialized_upvotes = bson::to_bson(&upvotes).unwrap(); //-- we have to serialize the upvotes to BSON Document object in order to update voters field inside the collection
            let serialized_downvotes = bson::to_bson(&downvotes).unwrap(); //-- we have to serialize the downvotes to BSON Document object in order to update voters field inside the collection
            match proposals.update_one(doc!{"_id": proposal_id}, doc!{"$set": { "voters": serialized_voters, "upvotes": serialized_upvotes, "downvotes": serialized_downvotes }}, None){
                Ok(updated_result) => {
                    let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                        message: UPDATED, //-- collection found in fishuman document (database)
                        status: 200,
                    };
                    Ok(
                        HttpResponse::Ok().json(
                            response_body
                        ).into_body() //-- call this method in order not to get failed to fetch in client side
                    )
                },
                Err(e) => {
                    let response_body = ctx::app::Response::<ctx::app::Nill>{
                        data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                        message: &e.to_string(), //-- take a reference to the string error
                        status: 500,
                    };
                    Ok(
                        HttpResponse::InternalServerError().json(
                            response_body
                        ).into_body() //-- call this method in order not to get failed to fetch in client side
                    )
                },
            }
        }, 
        None => { //-- means we didn't find any document related to this title and we have to tell the user to create a new proposaL
            let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                message: NOT_FOUND_DOCUMENT,
                status: 404,
            };
            Ok(
                HttpResponse::NotFound().json(
                    response_body
                ).into_body() //-- call this method in order not to get failed to fetch in client side
            )
        },
    }
    
}


#[post("/set-expire")]
async fn expire_proposal(req: HttpRequest, exp_info: web::Json<schemas::fishuman::ExpireProposalRequest>) -> Result<HttpResponse, Error>{
    
    // let storage = req.app_data::<web::Data<Option<Arc<ctx::app::Storage>>>>().unwrap(); //-- unwrapping the db inside the web data structure which is passed inside the app_data() method
    // let app_storage = match storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().mode{ //-- here as_ref() method convert &Option<T> to Option<&T>
    //     ctx::app::Mode::On => storage.as_ref().as_ref().unwrap().db.as_ref().unwrap().instance.as_ref(), //-- return the db if it wasn't detached - instance.as_ref() will return the Option<&Client>
    //     ctx::app::Mode::Off => None, //-- no db is available cause it's off
    // };

    let db_host = env::var("DB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("DB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let app_storage = Client::with_uri_str(db_addr).unwrap();

    let exp_info = exp_info.into_inner();
    let proposal_id = ObjectId::parse_str(exp_info._id.as_str()).unwrap(); //-- generating mongodb object id from the id string
    let proposals = app_storage.database("fishuman").collection::<schemas::fishuman::ProposalInfo>("proposals"); //-- selecting proposals collection to fetch all proposal infos into the ProposalInfo struct
    match proposals.find_one_and_update(doc!{"_id": proposal_id}, doc!{"$set": {"is_expired": true}}, None).unwrap(){ //-- finding proposal based on proposal id
        Some(proposal_doc) => { //-- deserializing BSON into the ProposalInfo struct
            let response_body = ctx::app::Response::<schemas::fishuman::ProposalInfo>{ //-- we have to specify a generic type for data field in Response struct which in our case is ProposalInfo struct
                data: Some(proposal_doc), //-- data is an empty &[u8] array
                message: UPDATED, //-- collection found in fishuman document (database)
                status: 200,
            };
            Ok(
                HttpResponse::Ok().json(
                    response_body
                ).into_body() //-- call this method in order not to get failed to fetch in client side
            )
        }, 
        None => { //-- means we didn't find any document related to this title and we have to tell the user to create a new proposaL
            let response_body = ctx::app::Response::<ctx::app::Nill>{ //-- we have to specify a generic type for data field in Response struct which in our case is Nill struct
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                message: NOT_FOUND_DOCUMENT,
                status: 404,
            };
            Ok(
                HttpResponse::NotFound().json(
                    response_body
                ).into_body() //-- call this method in order not to get failed to fetch in client side
            )
        },
    }

}








pub fn register(config: &mut web::ServiceConfig){
    config.service(add_proposal);
    config.service(cast_vote_proposal);
    config.service(expire_proposal);
    config.service(get_all_proposals);
}
