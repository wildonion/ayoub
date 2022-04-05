





use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId};






// NOTE - serializing from struct or json or bson into the utf8 bytes and deserializing from utf8 into json or struct or bson
// NOTE - to send some data back to the user we must serialize that data struct into the json and from there to utf8 to pass through the socket
// NOTE - to send fetched data from mongodb which is a bson object back to the user we must first deserialize the bson into its related struct and then serialize it to json to send back to the user through the socket





#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Simd{
    pub input: u32,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Voter{
    pub nft_owner_wallet_address: String,
    pub is_upvote: bool,
    pub score: u32, // NOTE - this is the number of event NFTs that this owner owns
}



#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CastVoteRequest{
    pub _id: String, //-- this is the id of the proposal took from the mongodb
    pub voter: Voter,
}










/*
  -----------------------------------------------------------------------------------------
| this struct will be used to deserialize event info bson from the mongodb into this struct
| -----------------------------------------------------------------------------------------
|
|
*/
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct EventInfo{
    pub _id: Option<ObjectId>,
    pub title: String,
    pub content: String,
    pub creator_wallet_address: Option<String>, //-- it might be None at initializing stage inside the add api
    pub upvotes: Option<u16>,
    pub downvotes: Option<u16>,
    pub voters: Option<Vec<Voter>>,
    pub phases: Option<Vec<Phase>>,
    pub is_expired: Option<bool>,
    pub expire_at: Option<i64>,
    pub created_at: Option<i64>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerInfo{
    pub _id: Option<ObjectId>, //-- this is the _id of the user from the users collection
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Phase{
    pub day: Vec<PlayerInfo>, //-- vector of all users at the end of the day that their status has changed
    pub mid_day: Vec<PlayerInfo>, //-- vector of all users at the end of the mid day that their status has changed
    pub night: Vec<PlayerInfo>, //-- vector of all users at the end of the night that their status has changed
}


/*
  -------------------------------------------------------------------------------------
| this struct will be used to deserialize event info json from client into this struct
| -------------------------------------------------------------------------------------
|
|
*/
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct EventAddRequest{
    pub title: String,
    pub content: String,
    pub creator_wallet_address: Option<String>,
    pub upvotes: Option<u16>, // NOTE - we set this field to Option cause we don't want to pass the upvotes inside the request body, we'll fill it inside the server
    pub downvotes: Option<u16>, // NOTE - we set this field to Option cause we don't want to pass the downvotes inside the request body, we'll fill it inside the server
    pub voters: Option<Vec<Voter>>, // NOTE - we set this field to Option cause we don't want to pass the voters inside the request body, we'll update it later on using cast-vote route
    pub is_expired: Option<bool>, // NOTE - we set this field to Option cause we don't want to pass the is_expired inside the request body, we'll update it once a proposal reached the deadline
    pub expire_at: Option<i64>, // NOTE - we set this field to Option cause we don't want to pass the expire_at inside the request body, we'll update it while we want to create a new proposal object
    pub created_at: Option<i64>, // NOTE - we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}


/*
  ------------------------------------------------------------------------------------------------------
| this struct will be used to put all available events in it and serialize as json to send back to user
| ------------------------------------------------------------------------------------------------------
|
|
*/
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct AvailableEvents{
    pub events: Vec<EventInfo>,
}


/*
  -------------------------------------------------------------------------------------
| this struct will be used to deserialize expire info json from client into this struct
| -------------------------------------------------------------------------------------
|
|
*/
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ExpireEventRequest{
    pub _id: String, //-- this is the id of the proposal took from the mongodb
}


/*
  -------------------------------------------------------------------------------------
| this struct will be used to deserialize delete info json from client into this struct
| -------------------------------------------------------------------------------------
|
|
*/
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct DeleteEventRequest{
    pub _id: String, //-- this is the id of the proposal took from the mongodb
}



impl EventInfo{

    pub async fn add_voter(self, voter: Voter) -> Vec<Voter>{ //-- we don't take a reference to self cause we can't dereference a shared reference and if we do that then cannot borrow `*voters` as mutable, cause it is behind a `&` reference and `voters` is a `&` reference, so the data it refers to cannot be borrowed as mutable cause we have to define the first argument as &mut self
        let mut voters = self.voters.unwrap();
        let index = voters.iter().position(|v| v.nft_owner_wallet_address == voter.nft_owner_wallet_address); //-- this owner has alreay voted to this proposal
        if index == None{
            voters.push(voter);
        }
        voters
    }
}