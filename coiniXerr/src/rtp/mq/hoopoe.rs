








use crate::*;
use utils::api; // macro apis for communicating with the ayoub hyper server hoopoe service like storing in db
use rtp::{
    rpc::server as rpc_server,
    wrtc::server as wrtc_server,
        ws::server as ws_server, // for chatapp
        socks::server as socks_server,
        p2p::udp::app as p2p_app,
    };
        









    
pub enum Topic{
    Hoop,
    ReHoop,
    Mention,
    HashTag,
    Like,
    AccountInfo,
}   



//// Account is the user that can publish and subscribe to the messages
pub struct Account{ 
    pub account_id: String, //// this is the _id of the account that wants to publish messages
    pub env: Environment,
    pub producer: Option<Producer<Dedup>>,
    pub consumer: Option<Consumer>,
} 

impl Account{

    pub async fn new(env: Environment, acc_id: String) -> Self{
        Self{
            account_id: acc_id,
            env,
            producer: None,
            consumer: None,
        }
    }

    pub async fn build_producer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

        let prod = self.env
                .producer()
                .name("hoopoe_publisher")
                .build("hoopoe_producer_stream")
                .await
                .unwrap();
        
        Self{
            account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
            env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
            producer: Some(prod),
            consumer: self.consumer, //// since self is not a shared reference thus we can move it into new scope
        }

    }

    pub async fn publish(self, topic: Topic, message: String){ //// we can't take a reference to self since the producer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.producer issue 


        // TODO - schedule old and new ones (from the last offline time) 
        //        to be executed from the hoops queue buffer until the consumer is backed on line
        // ...

        let body = match topic{
            Hoop => format!("hooping: {}", message), 
            ReHoop => format!("rehooping: {}", message), 
            Mention => format!("Mentioning: {}", message),
            HashTag => format!("Hashtaging: {}", message),
            Like => format!("Liking: {}", message),
        };

        
        //// if the first param of method was &self that means the instance is behind a shared reference
        //// but it can't be moved or cloned since Clone trait which is a supertrait of the Copy is not  
        //// implemented for DedUp thus we can't clone or move the self.producer out of the shared reference at all.
        if let Some(mut prod) = self.producer{
            prod
                .send(Message::builder().body(body).build(), |_| async move{})
                .await
                .unwrap();            
        }

    }

    pub async fn close_producer(self){
        self.producer
                .unwrap()
                .close().await
                .unwrap();
    }

    pub async fn close_consumer(self){
        let consumer_handler = self.consumer.unwrap().handle();
        consumer_handler
                .close().await
                .unwrap();
    }

    pub async fn build_consumer(self) -> Self{ //// we can't take a reference to self since the consumer field can't be moved out the shared reference due to not-implemented-Clone-trait-for-self.consumer issue

        let cons = self.env
                .consumer()
                .build("hoopoe_consumer_stream")
                .await
                .unwrap();
        
        Self{
            account_id: self.account_id.clone(), //// we're cloning the account_id since when we're creating the Self it'll move into a new instance scope
            env: self.env.clone(), //// we're cloning the env since when we're creating the Self it'll move into a new instance scope
            producer: self.producer, //// since self is not a shared reference thus we can move it into new scope
            consumer: Some(cons), 
        }

    }

    pub async fn subscribe(self){

        tokio::spawn(async move{
            while let Some(delivery) = self.consumer.unwrap().next().await{ //// streaming over the consumer to receive all the messages comming from the producer while there is some delivery
                info!("Received message {:?}", delivery);
            }
        });

    }

} 












//// list of all hoopoe publishers
pub struct HoopoePublishers{
    pub account: Vec<Account>,
}