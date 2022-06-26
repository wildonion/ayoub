





/*



Coded by



 █     █░ ██▓ ██▓    ▓█████▄  ▒█████   ███▄    █  ██▓ ▒█████   ███▄    █ 
▓█░ █ ░█░▓██▒▓██▒    ▒██▀ ██▌▒██▒  ██▒ ██ ▀█   █ ▓██▒▒██▒  ██▒ ██ ▀█   █ 
▒█░ █ ░█ ▒██▒▒██░    ░██   █▌▒██░  ██▒▓██  ▀█ ██▒▒██▒▒██░  ██▒▓██  ▀█ ██▒
░█░ █ ░█ ░██░▒██░    ░▓█▄   ▌▒██   ██░▓██▒  ▐▌██▒░██░▒██   ██░▓██▒  ▐▌██▒
░░██▒██▓ ░██░░██████▒░▒████▓ ░ ████▓▒░▒██░   ▓██░░██░░ ████▓▒░▒██░   ▓██░
░ ▓░▒ ▒  ░▓  ░ ▒░▓  ░ ▒▒▓  ▒ ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ ░▓  ░ ▒░▒░▒░ ░ ▒░   ▒ ▒ 
  ▒ ░ ░   ▒ ░░ ░ ▒  ░ ░ ▒  ▒   ░ ▒ ▒░ ░ ░░   ░ ▒░ ▒ ░  ░ ▒ ▒░ ░ ░░   ░ ▒░
  ░   ░   ▒ ░  ░ ░    ░ ░  ░ ░ ░ ░ ▒     ░   ░ ░  ▒ ░░ ░ ░ ▒     ░   ░ ░ 
    ░     ░      ░  ░   ░        ░ ░           ░  ░      ░ ░           ░ 
                      ░                                                  



*/




use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case









// NOTE - we can use #[serde(flatten)] attribute on a field of a struct or enum in those cases that we don't know about the exact keys or values inside the flattened field thus we can use this attribute to hold additional data that is not captured by any other fields of the struct or enum
// NOTE - since we don't know what's exactly inside the data of an event (cause it's an array of json) we have to flatten the event field inside the EventLog struct to only have the content of the current variant of EventLogVariant enum since this enum is tagged 










#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="event", content="data")] // NOTE - the deserialized data of the following enum  will be : {"event": "nft_mint", "data": [{...NftMintLog_instance...}, {...NftMintLog_instance...}]} or {"event": "nft_transfer", "data": [{...NftTransferLog_instance...}, {...NftTransferLog_instance...}]}
#[serde(rename_all="snake_case")] //-- converting all fields' name to snake_case format like nft_ming_log
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate
#[non_exhaustive] // NOTE - this attribute indicates that more variants/fields may be added to an enum/struct in the future and prevents downstream crates from exhaustively listing out all variants/fields
pub enum EventLogVariant{ //-- event log enum which can be either NFT mint or NFT transfer log 
    NftMint(Vec<NftMintLog>), //-- vector of all minting NFT events
    NftTransfer(Vec<NftTransferLog>), //-- vector of all transferring NFT events
}




#[derive(Serialize, Deserialize, Debug)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate
pub struct EventLog{ //-- an interface to capture the data abount and event - this is the EVENT_JSON 
    pub standard: String,
    pub version: String,
    #[serde(flatten)] //-- flatten to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>} which is the value of the data key itself - we can use #[serde(flatten)] attribute on a field of a struct or enum in those cases that we don't know about the number of exact fields inside the struct or enum or what's exactly inside the body of an api comming from the client to decode or map it into the struct or enum thus we can use this attribute to hold additional data that is not captured by any other fields of the struct or enum
    pub event: EventLogVariant, //-- the data which is a vector of all either NftMint or NftTransfer variant events - we'll have {"standard": "1", "version": "1", "event": "event name", "data": [{...NftMintLog_instance...}] or [{...NftTransferLog_instance...}]}
}




#[derive(Serialize, Deserialize, Debug)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate
pub struct NftMintLog{ //-- event log to capture token minting
    pub owner_id: AccountId, //-- the account_id of the minter who owns the NFT
    pub token_ids: Vec<TokenId>, //-- it might be a collection minting process!
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub memo: Option<String>,
}




#[derive(Serialize, Deserialize, Debug)]
#[serde(crate="near_sdk::serde")] //-- must be added right down below of the serde derive proc macro attributes - loading serde crate instance from near_sdk crate
pub struct NftTransferLog{ //-- event log to capture token transfer
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub authorized_id: Option<AccountId>, //-- if there was any approved account_id to transfer the NFT on behalf of the owner
    pub old_owner_id: AccountId,
    pub new_owner_id: AccountId,
    pub token_ids: Vec<TokenId>, //-- it might be a collection transferring process!
    #[serde(skip_serializing_if="Option::is_none")] //-- skip serializing this field if it was None
    pub memo: Option<String>,
}