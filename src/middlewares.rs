










pub mod auth{
    
    
    use crate::constants;
    use log::{info, error};
    use hyper::{Method, Body};
    use crate::utils::jwt;
    use jsonwebtoken::TokenData;

    
    

    pub async fn pass(req: hyper::Request<Body>) -> Result<(TokenData<jwt::Claims>, hyper::Request<Body>), jsonwebtoken::errors::Error>{
        let mut authenticate_pass: bool = false;
        let mut user_data_inside_token: Option<TokenData<jwt::Claims>> = None;
        let mut jwt_error: Option<jsonwebtoken::errors::Error> = None;
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else{
            for ignore_route in constants::IGNORE_ROUTES.iter(){
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
                                    authenticate_pass = true;
                                    user_data_inside_token = Some(token_data);
                                },
                                Err(e) => {
                                    jwt_error = Some(e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if authenticate_pass{
            Ok((user_data_inside_token.unwrap(), req))
        } else{
            Err(jwt_error.unwrap())
        }
    }


}