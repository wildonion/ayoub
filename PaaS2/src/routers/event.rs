




/*
    --------------------------------------------------------------------------
   |                      REGISTER HANDLER FOR EVENT ROUTER
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
use crate::controllers::event::{
                                add::main as add_event, 
                                get::{all as get_all_events, player_all as get_all_player_events, single as get_single_event}, 
                                vote::main as cast_vote_event, 
                                expire::main as expire_event, 
                                _404::main as not_found, 
                                phase::insert as insert_phase,
                                reserve::{process_payment_request, mock_reservation},
                                reveal::{role},
                                simd::main as simd_ops
                            };





pub async fn register() -> Router<Body, hyper::Error>{  



    Router::builder()
        .middleware(enable_cors_all())
        .post("/event/add", add_event)
        .get("/event/get/availables", get_all_events)
        .post("/event/get/all/player",get_all_player_events)
        .post("/event/get/single", get_single_event)
        .post("/event/cast-vote", cast_vote_event)
        .post("/event/set-expire", expire_event)
        .post("/event/update/phases/add", insert_phase)
        .post("/event/simd", simd_ops)
        .post("/event/reserve/mock", mock_reservation)
        .post("/event/reveal/roles", role)
        .options("/", middlewares::cors::send_preflight_response)
        .build()
        .unwrap()



}