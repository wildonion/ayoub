





pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
// pub type Result<T> = std::result::Result<T, GenericError>;
pub static INTERNAL_SERVER_ERROR: &str = "Interal Server Error";
pub static WRONG_CREDENTIALS: &str = "Wrong Credentials";
pub static WELCOME: &str = "Welcome Home";
pub static NOT_ACCEPTABLE: &str = "Not Acceptable";
pub static BAD_REQUEST: &str = "Bad Request";
pub static UNAUTHORISED: &str = "Unauthorised";
pub static METHOD_NOT_ALLOWED: &str = "Method Not Allowed";
pub static DEV_LEVEL: &u8 = &0; //-- DEV_LEVEL is of type &u8 so we have to set its value like &0
pub static ADMIN_LEVEL: &u8 = &1;
pub static USER_LEVEL: &u8 = &2;
pub static ACCESS_GRANTED: &str = "Access Granted";
pub static ACCESS_DENIED: &str = "Access Denied";
pub static REGISTERED: &str = "Registered Successfully";
pub static DO_LOGIN: &str = "Please Login";
pub static DO_SIGNUP: &str = "Please Signup";
pub static NOTFOUND_ROUTE: &str = "Not Found";
pub static SIMD_RESULT: &str = "Simd Result";
pub static FOUND_DOCUMENT: &str = "Found Document";
pub static INSERTED: &str = "Inserted Successfully";
pub static UPDATED: &str = "Updated Successfully";
pub static FETCHED: &str = "Fetched Successfully";
pub static DELETED: &str = "Deleted Successfully";
pub static NOT_FOUND_DOCUMENT: &str = "Not Found Document";
pub static NOT_FOUND_ROUTE: &str = "Not Found Route";
pub static IGNORE_ROUTES: &[&str] = &["login", "signup"];
pub static NOT_FOUND_TOKEN: &str = "No Token Found";
pub static NOT_IMPLEMENTED: &str = "Not Implemented";
pub static OTP_CODE_HAS_BEEN_SENT: &str = "OTP Code Has Been Sent Successfully";
pub static EXPIRED_OTP_CODE: &str = "OTP Code Has Been Expored";
pub const CHARSET: &[u8] = b"0123456789";
pub const DEFAULT_STATUS: &u8 = &0;
pub const KICK_STATUS: &u8 = &1;
pub const DEAD_STATUS: &u8 = &2;
pub const THREE_PHASES_DISABILITY_STATUS: &u8 = &3;
pub const FULL_DISABILITY_STATUS: &u8 = &4;
pub const TWO_PHASES_SILENT_STATUS: &u8 = &5;
pub const SIX_PHASES_SILENT_STATUS: &u8 = &6;
pub const CHAINED_STATUS: &u8 = &7;
pub const CHANGED_ROLL_STATUS: &u8 = &8;
pub const EXIT_STATUS: &u8 = &9;
pub const DEVOTE_STATUS: &u8 = &10;
pub const NINE_PHASES_LATER_PRO_KILLER_STATUS: &u8 = &11;
