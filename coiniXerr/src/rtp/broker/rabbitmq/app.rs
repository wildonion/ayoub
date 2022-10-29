






// we use multithreading design patterns like actors which use channels like mpsc to avoid race conditions to build multithreading jobq apps like rabbitmq
// the client app to write pub/sub codes like twitter tweets broadcaster and food ordering service on top of the following protocols


use crate::*;
use rtp::{
        rpc::server as rpc_server,
        wrtc::server as wrtc_server,
        ws::server as ws_server,
        socks::server as socks_server,
        p2p::udp::app as p2p_app,
    };