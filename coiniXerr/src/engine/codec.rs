





// TODO - codec for blockchain structures like borsh and serde which is for structs to utf8 or bson or json and vice versa to map from utf8 into struct; convert from struct into utf8 using a simple union
// NOTE - to send some data back to the user we must serialize that data struct into the json and from there to utf8 to pass through the socket
// NOTE - to send fetched data from mongodb which is a bson object back to the user we must first deserialize the bson into its related struct and then serialize it to json to send back to the user through the socket
// NOTE - borsh like codec ops : Box<[u8]> (automatic lifetime) or &'a [u8] <-> vec[u8] <-> struct
// NOTE - &[u8] bytes to str using std::from_utf8() -> parse it and build the key:value hashmap -> build the struct instance from the hashmap
// NOTE - deserialization using json string : &[u8] buffer ----serde_json::from_reader()----> Value ----serde_json::to_string()----> json string ----serde_json::from_str()----> struct
// NOTE - deserialization using slice       : &[u8] buffer ----serde_json::from_slice()----> struct



pub mod encoder;
pub mod decoder;
