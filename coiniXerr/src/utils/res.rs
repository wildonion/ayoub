






use crate::*; // loading all defined crates, structs and functions from the root crate which is lib.rs in our case



#[derive(Debug, Serialize, Deserialize)] //-- when we implement the Serialize trait for this struct we can put the instance of it inside the json() method cause we can serialize an instance of it into the json to send back to where it was called
pub struct ResponseBody<T>{
    pub message: String,
    pub data: T,
}

impl<T> ResponseBody<T>{
    pub fn new(message: &str, data: T) -> ResponseBody<T>{
        ResponseBody{
            message: message.to_string(),
            data,
        }
    }
}


pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}