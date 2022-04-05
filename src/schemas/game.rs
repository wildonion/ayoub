




use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;








// NOTE - serializing from struct or json or bson into the utf8 bytes and deserializing from utf8 into json or struct or bson
// NOTE - to send some data back to the user we must serialize that data struct into the json and from there to utf8 to pass through the socket
// NOTE - to send fetched data from mongodb which is a bson object back to the user we must first deserialize the bson into its related struct and then serialize it to json to send back to the user through the socket






#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct RoleInfo{
    pub _id: Option<ObjectId>, //-- ObjectId is the bson type of _id inside the mongodb
    pub name: String, //-- role name
    pub rate: u8, //-- role rate
    pub desc: String, //-- role description
    pub abilities: u8, //-- number of total abilities for a role, the default is 0
}


#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct DeckInfo{
    pub _id: Option<ObjectId>, //-- ObjectId is the bson type of _id inside the mongodb
    pub roles: RoleInfo,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct AvailableDecks{
    pub decks: Vec<DeckInfo>, //-- fetch all decks
}



#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct SidesInfo{
    pub _id: Option<ObjectId>, //-- ObjectId is the bson type of _id inside the mongodb
    pub name: String,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct PlayerRoleAbilityInfo{
    pub _id: Option<ObjectId>, //-- ObjectId is the bson type of _id inside the mongodb
    pub user_id: String, //-- string type of ObjectId for user id 
    pub role_id: String, //-- string type of ObjectId for role id
    pub event_id: String, //-- string type of ObjectId for event id
    pub current_ability: u8,
}


#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct PlayerChainToInfo{
    pub _id: Option<ObjectId>,
    pub from_id: String, //-- string type of ObjectId for from user id 
    pub to_id: String, //-- string type of ObjectId for to user id 

}