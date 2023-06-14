
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
// pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/ping", "/api/auth/signup", "/api/auth/login"];

// Default number of items per page
pub const DEFAULT_PER_PAGE: i64 = 10;

// Default page number
pub const DEFAULT_PAGE_NUM: i64 = 1;

pub const EMPTY_STR: &str = "";

pub const BIND: &str = "0.0.0.0:8080";

pub mod env_key {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const FRONTEND_ORIGIN: &str = "FRONTEND_ORIGIN";
}
