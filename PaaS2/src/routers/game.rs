



/*
    --------------------------------------------------------------------------
   |                      REGISTER HANDLER FOR GAME ROUTER
   |--------------------------------------------------------------------------
   |
   |    job    : the following registers a router requested by the client
   |    return : a Result of type either successful or error response
   |
   |

*/




use routerify::Router;
use routerify_cors::enable_cors_all;
use crate::middlewares;
use crate::contexts as ctx;
use hyper::{Method, Body, Response};
use std::sync::Arc;
use crate::controllers::game::{
                                role::{add as add_role, all as get_roles, disable as disable_role}, 
                                deck::{add as add_deck, all as get_decks, disable as disable_deck},
                                side::{add as add_side, all as get_sides, disable as disable_side}, 
                                player::{update_role_ability, chain_to_another_player, update_role, update_side, update_status, get_single, get_player_role_ability, get_player_chain_infos}, 
                                group::{create as create_group, update as update_group, all as get_groups, upload_img},
                                _404::main as not_found,
                            };





pub async fn register() -> Router<Body, hyper::Error>{  


    Router::builder()
        .middleware(enable_cors_all())
        .post("/game/role/add", add_role)
        .get("/game/role/get/availables", get_roles)
        .post("/game/role/disable", disable_role)
        .post("/game/deck/add", add_deck)
        .get("/game/deck/get/availables", get_decks)
        .post("/game/deck/disable", disable_deck)
        .post("/game/side/add", add_side)
        .get("/game/side/get/availables", get_sides)
        .post("/game/side/disable", disable_side)
        .post("/game/player/update/role", update_role)
        .post("/game/player/update/side", update_side)
        .post("/game/player/update/status", update_status)
        .post("/game/player/update/role-ability", update_role_ability)
        .post("/game/player/chain", chain_to_another_player)
        .post("/game/player/get/single", get_single)
        .post("/game/player/get/role-ability", get_player_role_ability)
        .post("/game/player/get/chain-infos", get_player_chain_infos)
        .post("/game/god/create/group", create_group)
        .post("/game/god/update/group/", update_group)
        .post("/game/god/update/group/image", get_groups)
        .get("/game/get/group/all", upload_img)
        .options("/", middlewares::cors::send_preflight_response)
        .build()
        .unwrap()


}