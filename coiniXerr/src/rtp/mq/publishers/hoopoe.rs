








use crate::*;
use utils::api; // for communicating with the ayoub hyper server hoopoe service
use rtp::{
    rpc::server as rpc_server,
    wrtc::server as wrtc_server,
        ws::server as ws_server, // for chatapp
        socks::server as socks_server,
        p2p::udp::app as p2p_app,
    };
        







//// schedule old and new ones (from the last offline time) to be executed from the hoops queue buffer until the consumer is backed on line
//// can publish payload with topis hoop, mention, hashtags and likes
pub struct AccountPublisher{ 
    pub account_id: String, //// this is the _id of the account
} 




//// list of all hoopoe publishers
pub struct HoopoePublishers{
    pub account: Vec<AccountPublisher>,
}