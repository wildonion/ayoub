











pub mod cors{




    use crate::constants::*;
    use log::{info, error};
    use hyper::{Method, Body};








    pub async fn send_preflight_response(req: hyper::Request<Body>) -> GenericResult<hyper::Response<Body>, hyper::Error>{
        let response = hyper::Response::builder() //-- building an empty response object with Access-Control-Allow-* enabled in its header
                                            .status(hyper::StatusCode::OK)
                                            .header("Access-Control-Allow-Origin", "*")
                                            .header("Access-Control-Allow-Headers", "*")
                                            .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
                                            .body(Body::default())
                                            .unwrap();

        Ok(response) //-- sending back an empty response to the browser to say that the preflight request was ok to get rid of the fucking CORS :)
    }




    pub async fn is_preflight_request(req: hyper::Request<Body>) -> Result<(bool, hyper::Request<Body>), (bool, hyper::Request<Body>)>{
        if Method::OPTIONS == req.method(){ //-- append Access-Control-Allow-Origin headers to the request before parsing its body
            Ok((true, req))
        } else{
            Err((false, req))
        }
    }


}

















pub mod auth{
    
    
    use crate::constants::*;
    use log::{info, error};
    use hyper::{Method, Body};
    use crate::utils::jwt;
    use jsonwebtoken::TokenData;
    







    


    pub async fn pass(req: hyper::Request<Body>) -> Result<(TokenData<jwt::Claims>, hyper::Request<Body>), String>{ //-- the return type is a Result of type TokenData claims and hyper::Request body
        let mut authenticate_pass: bool = false;
        let mut user_data_inside_token: Option<TokenData<jwt::Claims>> = None;
        let mut jwt_error: Option<jsonwebtoken::errors::Error> = None;
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else{
            for ignore_route in IGNORE_ROUTES.iter(){
                if req.uri().path().starts_with(ignore_route){
                    authenticate_pass = true;
                    break;
                }
            }
            if !authenticate_pass{
                if let Some(authen_header) = req.headers().get("Authorization"){
                    if let Ok(authen_str) = authen_header.to_str(){
                        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer"){
                            let token = authen_str[6..authen_str.len()].trim();
                            match jwt::deconstruct(token).await{
                                Ok(token_data) => {
                                    authenticate_pass = true; //-- means we've found the token inside the request header and decoded successfully 
                                    user_data_inside_token = Some(token_data);
                                },
                                Err(e) => {
                                    jwt_error = Some(e);
                                }
                            }
                        }
                    }
                } else{
                    return Err(NOT_FOUND_TOKEN.to_string());
                }
            }
        }
        if authenticate_pass{
            Ok((user_data_inside_token.unwrap(), req)) //-- since we can't copy or clone the req object we have to return the request object back to where it has been called 
        } else{
            Err(jwt_error.unwrap().to_string())
        }
    }





    pub mod user{
        
        use hyper::Body;
        use crate::schemas;
        use mongodb::{Client, bson::{self, doc, oid::ObjectId}}; //-- self referes to the bson struct itself cause there is a struct called bson inside the bson.rs file

        

        pub async fn exists(storage: Option<&Client>, user_id: Option<ObjectId>, username: String, access_level: u8) -> bool{
    
            ////////////////////////////////// DB Ops
    
            let serialized_access_level = bson::to_bson(&access_level).unwrap(); //-- we have to serialize the access_level to BSON Document object in order to find a user with this info cause mongodb can't do serde ops on raw u8
            let users = storage.unwrap().database("ayoub").collection::<schemas::auth::UserInfo>("users"); //-- selecting users collection to fetch all user infos into the UserInfo struct
            match users.find_one(doc!{"_id": user_id, "username": username, "access_level": serialized_access_level}, None).await.unwrap(){ //-- finding user based on username, _id and access_level
                Some(user_doc) => true, 
                None => false,
            }
    
            //////////////////////////////////
     
        }


    }


}