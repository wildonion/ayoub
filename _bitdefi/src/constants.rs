





pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
// pub type Result<T> = std::result::Result<T, GenericError>;
pub static INTERNAL_SERVER_ERROR: &str = "Interal Server Error";
pub static WELCOME: &str = "Welcome Home";
pub static FORBIDDEN: &str = "Forbidden";
pub static NOT_FOUND_ROUTE: &str = "Not Found Route";
pub static NOT_ACCEPTABLE: &str = "Not Acceptable";
pub static BAD_REQUEST: &str = "Bad Request";
pub static METHOD_NOT_ALLOWED: &str = "Method Not Allowed";
pub static NOT_FOUND_DOCUMENT: &str = "Not Found Document";
pub static FOUND_DOCUMENT: &str = "Found Document";
pub static INSERTED: &str = "Inserted Successfully";
pub static UPDATED: &str = "Updated Successfully";
pub static FETCHED: &str = "Fetched Successfully";