





// TODO - build discord bot for ayoub PaaS
// ...
// https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/
// https://betterprogramming.pub/writing-a-discord-bot-in-rust-2d0e50869f64






#[derive(Clone, Debug)]
pub struct LoadBalancer; // TODO - clients -request-> middleware server -request-> main servers



pub mod manager{
    
    use uuid::Uuid;

    // TODO - vector of || async move{} of events for an event manager struct 
    // TODO - call new event every 5 seconds from vector of event of closures 
    
    pub struct Event{
        id: Uuid,
        last_call: i64, // last call timestamp  
    }


}


pub mod messanger{
    
    
    use uuid::Uuid;

    
    // TODO - simd using borsh and serde codec and actix actor based cross sharding
    // TODO - use actix actors for each server
    // ....
    
    pub struct Server<'a>{ //-- 'a is the lifetime of &[u8] which is the borrowed type of [u8] due to its unknown size at compile time  
        pub cluster_id: Uuid, //-- the id of the cluster which this server is inside
        pub api_token: &'a [u8], //-- is an array of a borrowed type of utf8 bytes with a valid lifetime 
        pub name: String,
        pub channels: Vec<Channel>,
        pub members: Vec<ServerMember>,
    }
    
    pub struct Thread{
        pub id: Uuid,
        pub name: String,
    }
    
    pub struct Channel{
        pub name: String,
        pub members: Vec<ChannelMember>,
        pub threads: Vec<Thread>,
    }
    
    pub struct ServerMember;
    pub struct ChannelMember;
        


}


pub trait void{
    type Input;
    type Output;

}