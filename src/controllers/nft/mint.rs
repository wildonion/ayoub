






use crate::contexts as ctx;
use crate::constants::*;
use futures::{executor::block_on, TryFutureExt, TryStreamExt}; //-- futures is used for reading and writing streams asyncly from and into buffer using its traits and TryStreamExt trait is required to use try_next() method on the future object which is solved by .await - try_next() is used on futures stream or chunks to get the next future IO stream and returns an Option in which the chunk might be either some value or none
use bytes::Buf; //-- it'll be needed to call the reader() method on the whole_body buffer and is used for manipulating coming network bytes from the socket
use hyper::{header, StatusCode, Body, Response};
use log::info;
use mongodb::{Client, bson::{self, doc, oid::ObjectId}}; //-- self referes to the bson struct itset cause there is a struct called bson inside the bson.rs file











// -------------------------------- mint controller
//
// -------------------------------------------------------------------------

pub async fn main(db: Option<&Client>, api: ctx::app::Api) -> GenericResult<hyper::Response<Body>, hyper::Error>{
    
    info!("calling {} - {}", api.name, chrono::Local::now().naive_local());
    
    api.post("/event/add", |req, res| async move{ // NOTE - api will be moved here cause neither trait Copy nor Clone is not implemented for that
        
        
        
        
        // calling mint method of the deployed bluerangene contract 
        // ...
        // near call $NFT_CONTRACT_ID nft_mint '{"token_id": "token-1", "metadata": {"title": "My Non Fungible Team Token", "description": "The Team Most Certainly Goes :)", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "'$NFT_CONTRACT_ID'"}' --accountId $NFT_CONTRACT_ID --amount 0.1
        


        let res = Response::builder(); //-- creating a new response cause we didn't find any available route
        let response_body = ctx::app::Response::<ctx::app::Nill>{
            message: NOTFOUND_ROUTE,
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
    }).await
    
}
