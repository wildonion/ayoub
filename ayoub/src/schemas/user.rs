




use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use argon2::{self, Config};
use std::env;






/*
  ----------------------------------------------------------------------------------------
| this struct will be used to deserialize register info json from client into this struct
| ----------------------------------------------------------------------------------------
|
|
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterRequest{ // NOTE - those Option values can be None tho
    pub username: String,
    pub phone: String,
    pub pwd: String, //-- hashed password
    pub role: String, // NOTE - dev, admin, user
    pub status: u8, // NOTE - 1 means active ; 0 means suspended
    pub created_at: Option<chrono::NaiveDateTime>, //-- we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}


/*
  -------------------------------------------------------------------------------------
| this struct will be used to deserialize login info json from client into this struct
| -------------------------------------------------------------------------------------
|
|
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest{ // NOTE - those Option values can be None tho
    pub username: String,
    pub pwd: String,
}


/*
  ----------------------------------------------------------------------------------------
| this struct will be used to deserialize user info bson from the mongodb into this struct
| ----------------------------------------------------------------------------------------
|
|
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo{ // NOTE - those Option values can be None tho
    pub _id: Option<ObjectId>,
    pub username: String,
    pub pwd: String,
    pub phone: String,
    pub role: String, // NOTE - dev, admin, user
    pub status: u8, // NOTE - 1 means active ; 0 means suspended
    pub created_at: Option<chrono::NaiveDateTime>,
}


/*
  --------------------------------------------------------------------------------------------------------------------------
| this struct will be used to deserialize user info bson from the mongodb into this struct to serialize as json back to user
| --------------------------------------------------------------------------------------------------------------------------
|
|
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginResponse{ // NOTE - those Option values can be None tho
    pub _id: Option<ObjectId>,
    pub access_token: String,
    pub username: String,
    pub phone: String,
    pub role: String, // NOTE - dev, admin, user
    pub status: u8, // NOTE - 1 means active ; 0 means suspended
    pub created_at: Option<chrono::NaiveDateTime>, //-- we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}


/*
  --------------------------------------------------------------------------------------------------------------------------
| this struct will be used to deserialize user info bson from the mongodb into this struct to serialize as json back to user
| --------------------------------------------------------------------------------------------------------------------------
|
|
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterResponse{ // NOTE - those Option values can be None tho
    pub _id: Option<ObjectId>,
    pub username: String,
    pub phone: String,
    pub role: String, // NOTE - dev, admin, user
    pub status: u8, // NOTE - 1 means active ; 0 means suspended
    pub created_at: Option<chrono::NaiveDateTime>, //-- we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}


/*
  ------------------------------------------------------------------------------------
| this struct will be used to deserialize token info json from client into this struct
| ------------------------------------------------------------------------------------
|
|
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenRequest{
    pub access_token: String,
}


/*
  --------------------------------------------------------------------------------------------------------------------------
| this struct will be used to deserialize user info bson from the mongodb into this struct to serialize as json back to user
| --------------------------------------------------------------------------------------------------------------------------
|
|
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckTokenResponse{ // NOTE - those Option values can be None tho
    pub _id: Option<ObjectId>,
    pub username: String,
    pub phone: String,
    pub role: String, // NOTE - dev, admin, user
    pub status: u8, // NOTE - 1 means active ; 0 means suspended
    pub created_at: Option<chrono::NaiveDateTime>, //-- we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}

impl RegisterRequest{

    pub async fn hash_pwd(raw_password: String) -> Result<String, argon2::Error>{
        let salt = env::var("SECRET_KEY").expect("⚠️ no secret key variable set");
        let salt_bytes = salt.as_bytes();
        let password_bytes = raw_password.as_bytes();
        let config = Config::default();
        argon2::hash_encoded(password_bytes, salt_bytes, &config)
    }

}


impl LoginRequest{

    pub async fn verify_pwd(hashed_password: String, raw_password: String) -> Result<bool, argon2::Error>{
        let password_bytes = raw_password.as_bytes();
        Ok(argon2::verify_encoded(&hashed_password, password_bytes).unwrap())
    }

}