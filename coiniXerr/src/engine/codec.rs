





// TODO - codec for blockchain structures like borsh and serde which is for structs to utf8 or bson or json and vice versa
// NOTE - map from utf8 into struct; convert from struct into utf8 using a simple union
// NOTE - serializing from struct or json or bson into the utf8 bytes and deserializing from utf8 into json or struct or bson
// NOTE - to send some data back to the user we must serialize that data struct into the json and from there to utf8 to pass through the socket
// NOTE - to send fetched data from mongodb which is a bson object back to the user we must first deserialize the bson into its related struct and then serialize it to json to send back to the user through the socket
// NOTE - borsh like codec ops : Box<[u8]> (automatic lifetime) or &'a [u8] <-> vec[u8] <-> struct
// NOTE - &[u8] bytes to str using std::from_utf8() -> parse it and build the hashmap -> map the hashmap key and value str into the struct  



pub mod encoder;
pub mod decoder;
