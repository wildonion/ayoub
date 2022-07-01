






use std::{collections::HashMap, sync::{Arc, Mutex}};
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fmt;




#[derive(Serialize, Deserialize, BorshSerialize)]
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


    // println!(">>>>>>>>>>>>> storage key for the TokensPerOwner {:#?}", Storagekey::TokensPerOwner.try_to_vec().unwrap()); ////// 0
    // println!(">>>>>>>>>>>>> storage key for the TokensById {:#?}", Storagekey::TokensById.try_to_vec().unwrap()); ////// 2
    // println!(">>>>>>>>>>>>> storage key for the TokenMetadataById {:#?}", Storagekey::TokenMetadataById.try_to_vec().unwrap()); ////// 3



    // println!("{:#?}", first);
    // println!("{:#?}", second);






    #[derive(Serialize, Deserialize, Debug)]
    struct Pagination {
        limit: u64,
        offset: u64,
        total: u64,
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    struct Users {
        users: Vec<User>,
        #[serde(flatten)]
        pagination: Pagination,
    }


    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        id: String,
        username: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    }


    #[derive(Serialize, Deserialize, Debug)]
    struct An {
        id: String,
        username: String,
    }


    // -----------------------
    let mut extra: HashMap<String, Value> = HashMap::new(); // it can be contain any
    let j = "
    {
        \"id\": \"0xF9BA143B95FF6D82\",
        \"username\": \"Menlo Park, CA\"
    }";

    let u: An = serde_json::from_str(j).unwrap();
    let value = serde_json::to_value(u).unwrap();
    extra.insert("testy-test".to_string(), value);
    let user = User{
        id: "id".to_string(),
        username: "wildonion".to_string(),
        extra,
    };
    let pg = Pagination{
        limit: 23,
        offset: 213,
        total: 123,
    };
    // -----------------------


    let users = Users{
        users: vec![user],
        pagination: pg,  
    };

    println!("{:?}", users.to_string());
    let serialized = serde_json::to_vec(&users).unwrap();
    let deserialize = serde_json::from_slice::<Users>(&serialized).unwrap();
    println!("{:?}", deserialize.to_string());
    

    


    impl fmt::Display for Users{ //-- implementing the Display trait for the EventLog struct to show its instances' fields like EVENT_JSON:{"time": 167836438974, "event": "event name, "data": [{...RuntimeLog_instance...}] or [{...ServerlessLog_instance...}]} when we're calling logging functions which is a formatted stream of strings - any value or type that implements the Display trait can be passed to format_args!() macro, as can any Debug implementation be passed to a {:?} within the formatting string; Debug must be implemented for the type
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
            f.write_fmt( //-- writing some formatted information using format_args!() macro into the formatter instance which is `f`
                format_args!( //-- format_args!(), unlike its derived macros, avoids heap allocations
                    "EVENT_JSON:{}", //-- it'll start with EVENT_JSON:{}
                    &serde_json::to_string(self).map_err(|_| fmt::Error).unwrap() //-- formatting every field of the self which is the instance of the EventLog struct into the string to writ into the `f` and catch the fmt::error of each message or field if there was any when we're creating the stream by formatting the struct
                ) 
            )
        }
    }




    #[derive(Serialize, Deserialize)]
    enum Chie{
        Avali(u8),
        Dovomi(String),
    }


    let ine = Chie::Avali(12);

    match ine{
        Chie::Avali(value) if value == 23 => {
            println!("u8 eeee");

        },
        Chie::Dovomi(value) if value == "wildonion".to_string() => {
            println!("stringeeee");
        },
        _ => {
            println!("none of them");
        }
    }














}
