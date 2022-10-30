






// publishing queues:
//  - hoops (schedule)
//  - mentions
//  - hashtags
//  - likes
//  - broadcast or publish to whole hoopoe app





use crate::*;
use rtp::{
    rpc::server as rpc_server,
    wrtc::server as wrtc_server,
        ws::server as ws_server,
        socks::server as socks_server,
        p2p::udp::app as p2p_app,
    };
        
    

pub struct HoopoePublisher;