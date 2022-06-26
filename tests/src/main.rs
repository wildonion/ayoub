






use std::{collections::HashMap, sync::{Arc, Mutex}};
use borsh::{BorshDeserialize, BorshSerialize};





#[derive(BorshSerialize)]
pub enum Storagekey{
    TokensPerOwner, 
    TokenPerOwnerInner{account_id_hash: [u8; 2]},
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner{token_type_hash: [u8; 2]},
    TokenTypesLocked,
}








fn main() {
   



    let message = Arc::new(
                                                Mutex::new(
                                                        Storagekey::TokenPerOwnerInner{ 
                                                            account_id_hash: [23, 24] 
                                                        }
                                                    )
                                                );

    let mut first: HashMap<u8, String> = HashMap::new();
    let mut second: HashMap<u8, String> = HashMap::new();


    first.insert(1, "wildonion".to_string());
    second.insert(1, "wildonion".to_string());


    println!(">>>>>>>>>>>>> storage key for the TokensPerOwner {:#?}", Storagekey::TokensPerOwner.try_to_vec().unwrap()); ////// 0
    println!(">>>>>>>>>>>>> storage key for the TokensById {:#?}", Storagekey::TokensById.try_to_vec().unwrap()); ////// 2
    println!(">>>>>>>>>>>>> storage key for the TokenMetadataById {:#?}", Storagekey::TokenMetadataById.try_to_vec().unwrap()); ////// 3



    println!("{:#?}", first);
    println!("{:#?}", second);



    pub struct Can{
        pub status: u8,
    };
    
    impl Can{
        pub fn new() -> Can{
            Can{
                status: 23,
            }
        }
    }

    pub fn ret_ref(status: &u8) -> &Can{
        let t = Can::new();
        // &t
        let c = &Can{status: 23};
        c

    }












    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};
    use serde_json::Value;

    #[derive(Serialize, Deserialize)]
    struct User {
        id: String,
        username: String,

        #[serde(flatten)]
        extra: HashMap<String, Value>,
    }


































}
