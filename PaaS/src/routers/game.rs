



/*
    --------------------------------------------------------------------------
   |                      REGISTER HANDLER FOR GAME ROUTER
   |--------------------------------------------------------------------------
   |
   |    job    : the following registers a router requested by the client
   |    return : a Router of type either hyper response body or error response
   |
   |
   |
   | we don't need to have one response object for each router and we can build
   | a new one inside the body of each router since rust doesn't support garbage
   | collection rule and each response object will be dropped once each router 
   | router body scope gets ended.
   |
   |
   | instead of initializing the app_storage inside each router api we've 
   | initialized it only once per router to move it between each router api.
   | 

*/




use std::env;
use mongodb::Client;
use routerify::{Router, Middleware};
use routerify_cors::enable_cors_all;
use crate::middlewares;
use crate::constants::*;
use crate::contexts as ctx;
use hyper::{header, Body, Response, StatusCode};
use crate::controllers::game::{
                                role::{add as add_role, all as get_roles, disable as disable_role}, 
                                deck::{add as add_deck, all as get_decks, disable as disable_deck, single as get_single_deck},
                                side::{add as add_side, all as get_sides, disable as disable_side}, 
                                player::{update_role_ability, chain_to_another_player, update_role, update_side, update_status, get_single, get_player_role_ability, get_player_chain_infos}, 
                                group::{create as create_group, update as update_group, all as get_groups, upload_img},
                                _404::main as not_found,
                            };





pub async fn register() -> Router<Body, hyper::Error>{  



    let db_host = env::var("MONGODB_HOST").expect("⚠️ no db host variable set");
    let db_port = env::var("MONGODB_PORT").expect("⚠️ no db port variable set");
    let db_engine = env::var("DB_ENGINE").expect("⚠️ no db engine variable set");
    let db_addr = format!("{}://{}:{}", db_engine, db_host, db_port);
    let app_storage = Client::with_uri_str(&db_addr).await.unwrap();

    ////////
    // NOTE - only the request object must be passed through each handler
    ////////


    Router::builder()
        .data(app_storage) //-- sharing the initialized app_storage between routers' threads
        .middleware(Middleware::post(middlewares::cors::allow))
        .middleware(Middleware::pre(middlewares::logging::logger)) //-- enable logging middleware on the incoming request then pass it to the next middleware
        .get("/page", |req| async move{
            let res = Response::builder(); //-- creating a new response cause we didn't find any available route
            let response_body = ctx::app::Response::<ctx::app::Nill>{
                message: WELCOME,
                data: Some(ctx::app::Nill(&[])), //-- data is an empty &[u8] array
                status: 200,
            };
            let response_body_json = serde_json::to_string(&response_body).unwrap(); //-- converting the response body object into json stringify to send using hyper body
            Ok(
                res
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(response_body_json)) //-- the body of the response must be serialized into the utf8 bytes to pass through the socket
                    .unwrap()
            )
        })
        .post("/role/add", add_role)
        .get("/role/get/availables", get_roles)
        .post("/role/disable", disable_role)
        .post("/deck/add", add_deck)
        .get("/deck/get/availables", get_decks)
        .post("/deck/get/single", get_single_deck)
        .post("/deck/disable", disable_deck)
        .post("/side/add", add_side)
        .get("/side/get/availables", get_sides)
        .post("/side/disable", disable_side)
        .post("/player/update/role", update_role)
        .post("/player/update/side", update_side)
        .post("/player/update/status", update_status)
        .post("/player/update/role-ability", update_role_ability)
        .post("/player/chain", chain_to_another_player)
        .post("/player/get/single", get_single)
        .post("/player/get/role-ability", get_player_role_ability)
        .post("/player/get/chain-infos", get_player_chain_infos)
        .post("/god/create/group", create_group)
        .post("/god/update/group/", update_group)
        .post("/god/update/group/image", upload_img)
        .get("/get/group/all", get_groups)
        .any(not_found) //-- handling 404 request
        .build()
        .unwrap()


}