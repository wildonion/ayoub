




use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use argon2::{self, Config};
use std::env;





#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegInfo{ // NOTE - those Option values can be None tho
    pub username: String,
    pub phone: String,
    pub pwd: String, //-- hashed password
    pub role: String, // NOTE - dev, admin, user
    pub status: u8, // NOTE - 1 means active ; 0 means suspended
    pub created_at: Option<chrono::NaiveDateTime>, //-- we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginInfo{ // NOTE - those Option values can be None tho
    pub username: String,
    pub pwd: String,
}


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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponse{ // NOTE - those Option values can be None tho
    pub _id: Option<ObjectId>,
    pub username: String,
    pub phone: String,
    pub role: String, // NOTE - dev, admin, user
    pub status: u8, // NOTE - 1 means active ; 0 means suspended
    pub created_at: Option<chrono::NaiveDateTime>, //-- we set this field to Option cause we don't want to pass the created time inside the request body, we'll fill it inside the server
}


impl RegInfo{

    pub async fn hash_pwd(raw_password: String) -> Result<String, argon2::Error>{
        let salt = env::var("SECRET_KEY").expect("⚠️ no secret key variable set");
        let salt_bytes = salt.as_bytes();
        let password_bytes = raw_password.as_bytes();
        let config = Config::default();
        argon2::hash_encoded(password_bytes, salt_bytes, &config)
    }

}


impl LoginInfo{

    pub async fn verify_pwd(hashed_password: String, raw_password: String) -> Result<bool, argon2::Error>{
        let password_bytes = raw_password.as_bytes();
        Ok(argon2::verify_encoded(&hashed_password, password_bytes).unwrap())
    }

}