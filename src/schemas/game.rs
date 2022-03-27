





// NOTE - serializing from struct or json or bson into the utf8 bytes and deserializing from utf8 into json or struct or bson
// NOTE - to send some data back to the user we must serialize that data struct into the json and from there to utf8 to pass through the socket
// NOTE - to send fetched data from mongodb which is a bson object back to the user we must first deserialize the bson into its related struct and then serialize it to json to send back to the user through the socket






pub struct Decks{ // [ { _id, roles{ _id, name, rate, desc, abilities} }, ..., { _id, roles{ _id, name, rate, desc, abilities} } ]

}

pub struct Sides{ // _id, name

}

pub struct PlayerRoleAbility{ // _id, user_id, role_id, event_id, current_ability

}


pub struct PlayerChainTo{ // _id, from_id, to_id 

}