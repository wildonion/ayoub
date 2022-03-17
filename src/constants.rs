





pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
// pub type Result<T> = std::result::Result<T, GenericError>;
pub static INTERNAL_SERVER_ERROR: &str = "Interal Server Error";
pub static WRONG_CREDENTIALS: &str = "Wrong Credentials";
pub static ACCESS_GRANTED: &str = "Access Granted";
pub static ACCESS_DENIED: &str = "Access Granted";
pub static REGISTERED: &str = "Registered Successfully";
pub static DO_LOGIN: &str = "Please Login";
pub static DO_SIGNUP: &str = "Please Signup";
pub static WELCOME: &str = "Welcome Home";
pub static NOTFOUND_ROUTE: &str = "Not Found";
pub static NOT_ACCEPTABLE: &str = "Not Acceptable";
pub static BAD_REQUEST: &str = "Bad Request";
pub static UNAUTHORISED: &str = "Unauthorised";
pub static METHOD_NOT_ALLOWED: &str = "Method Not Allowed";
pub static DEV_LEVEL: &u8 = &0; //-- DEV_LEVEL is of type &u8 so we have to set its value like &0
pub static ADMIN_LEVEL: &u8 = &1;
pub static USER_LEVEL: &u8 = &2;
pub static SIMD_RESULT: &str = "Simd Result";
pub static FOUND_DOCUMENT: &str = "Found Document";
pub static INSERTED: &str = "Inserted Successfully";
pub static UPDATED: &str = "Updated Successfully";
pub static FETCHED: &str = "Fetched Successfully";
pub static NOT_FOUND_DOCUMENT: &str = "Not Found Document";
pub static NOT_FOUND_ROUTE: &str = "Not Found Route";