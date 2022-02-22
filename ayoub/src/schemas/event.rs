








use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId};


// NOTE - a mongodb document is serialized into the BSON format



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voter{
    pub fishuman_owner_wallet_address: String,
    pub is_upvote: bool,
    pub score: u32, // NOTE - this is the number of fishumans that this owner owns
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CastVoteRequest{
    pub _id: String, //-- this is the id of the proposal took from the mongodb
    pub voter: Voter,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProposalAddRequest{
    pub title: String,
    pub content: String,
    pub creator_wallet_address: String,
    pub upvotes: Option<u16>, // NOTE - we set this field to Option cause we don't want to pass the upvotes inside the request body, we'll fill it inside the server
    pub downvotes: Option<u16>, // NOTE - we set this field to Option cause we don't want to pass the downvotes inside the request body, we'll fill it inside the server
    pub voters: Option<Vec<Voter>>, // NOTE - we set this field to Option cause we don't want to pass the voters inside the request body, we'll update it later on using cast-vote route
    pub is_expired: Option<bool>, // NOTE - we set this field to Option cause we don't want to pass the is_expired inside the request body, we'll update it once a proposal reached the deadline
    pub expire_at: Option<i64>, // NOTE - we set this field to Option cause we don't want to pass the expire_at inside the request body, we'll update it while we want to create a new proposal object
    pub created_at: Option<i64>, // NOTE - we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProposalInfo{
    pub _id: Option<ObjectId>,
    pub title: String,
    pub content: String,
    pub upvotes: Option<u16>,
    pub downvotes: Option<u16>,
    pub voters: Option<Vec<Voter>>,
    pub is_expired: Option<bool>,
    pub expire_at: Option<i64>,
    pub created_at: Option<i64>,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvailableProposals{
    pub proposals: Vec<ProposalInfo>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpireProposalRequest{
    pub _id: String, //-- this is the id of the proposal took from the mongodb
}


impl ProposalInfo{

    pub async fn add_voter(self, voter: Voter) -> Vec<Voter>{
        let mut voters = self.voters.unwrap();
        voters.push(voter);
        voters
    }
}
