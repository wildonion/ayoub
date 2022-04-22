





use crate::*; // load all defined crates, structs and functions from the root crate which is lib.rs in our case





pub type TokenId = String;





#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- loading serde crate instance from near_sdk crate
pub struct Payout{ //-- payout type for the royalty standards
    pub payout: HashMap<AccountId, U128>, // NOTE - HashMap has loaded inside the lib.rs before and we imported using use crete::* syntax 
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate="near_sdk::serde")] //-- loading serde crate instance from near_sdk crate
pub struct NFTContractMetadata{ //-- token metadata info at contract level
    pub spec: String, //-- required, nft contract metadata version
    pub name: String, //-- required, nft contract metadata name
    pub symbol: String, //-- required, nft contract metadata symbol
    pub icon: Option<String>, //-- optional, nft contract metadata icon (cost storage)
    pub base_uri: Option<String>, //-- optional, nft contract metadata url to decentralized storage of the assets referenced by `reference` or `media` url fields
    pub reference: Option<String>, //-- optional, nft contract metadata url to a json file which contains more info about the asset
    pub reference_hash: Option<Base64VecU8>, //-- optional, a base64 string encoded version of sha256 hash of the json from the reference field which is in form Vec<u8>; is more like: base64(Vec<u8>(reference_field)) - required if the reference field is required
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")] //-- loading serde crate instance from near_sdk crate
pub struct TokenMetadata{ //-- token metadata info at token level itself
    pub title: Option<String>, //-- optional, token metadata title
    pub description: Option<String>, //-- optional, token metadata description
    pub media: Option<String>, //-- optional, token metadata url to decentralized storage of the media content
    pub media_hash: Option<Base64VecU8>, //-- optional, a base64 string encoded version of sha256 hash of the media content from the media field which is in form Vec<u8>; is more like: base64(Vec<u8>(media_field)) - required if the media field is required
    pub copies: Option<u64>, //-- optional, number of copies of this set of metadata in existence when token was minted
    pub issued_at: Option<u64>, //-- optional, token metadata unix timestamp of the minted token
    pub expires_at: Option<u64>, //-- optional, token metadata unix timestamp of the minted token
    pub updated_at: Option<u64>, //-- optional, token metadata unix timestamp of the updated time of this token
    pub extra: Option<String>, //-- optional, extra on chain info about the nft and it can be stringified json
    pub reference: Option<String>, //-- optional, token metadata url to a json file which contains more info about the asset
    pub reference_hash: Option<Base64VecU8>, //-- optional, a base64 string encoded version of sha256 hash of the json from the reference field which is in form Vec<u8>; is more like: base64(Vec<u8>(reference_field)) - required if the reference field is required
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token{
    pub owner_id: AccountId, //-- owner of the token
}


#[derive(Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct JsonToken{ //-- the token json info which will be returned from view calls
    pub owner_id: AccountId, //-- the owner of the token
    pub token_id: TokenId, //-- the id of the token which is of type String
    pub metadata: TokenMetadata, //-- the metadata of the token
}


pub trait NoneFungibleTokenMetadata{ //-- defining an object safe trait for NFT metadata queries, we'll implement this for any contract that wants to interact with NFT metadata queries - object safe traits are not bounded to trait Sized thus they won't return Self or have generic params in its methods if so then some space shoul have been allocated inside the memory for Self or that generic param and it wasn't no longer an abstract type  
    fn nft_metadata(&self) -> NFTContractMetadata; //-- the return type is of type NFTContractMetadata struct - we should borrow the self (&self) as far as we can
}


#[near_bindgen] //-- implementing the near_bindgen attribute on the trait implementation for the Contract struct in order to have a compiled trait for this struct 
impl NoneFungibleTokenMetadata for Contract{ //-- implementing the NoneFungibleTokenMetadata trait for our main Contract struct; bounding the mentioned trait to the Contract struct to query NFT metadata infos
    fn nft_metadata(&self) -> NFTContractMetadata{ //-- overriding the nft_metadata() method
        self.metadata.get().unwrap() //-- since metadata field is inside the LazyOption we must get the actual data itself using get() method which will return the type (NFTContractMetadata in our case) inside an Option
    }
}