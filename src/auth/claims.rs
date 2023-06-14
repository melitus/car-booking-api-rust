use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    /// Subject of the token -- whom token refers to.  The user id in our case.
    pub sub: Uuid,
    // issued at (UNIX timestamp)
    pub iat: i64,
    // expiry time (UNIX timestamp)
    pub exp: i64,
    // not before (UNIX timestamp)
    pub nbf: i64,
    // issuer (Who issued the token)
    pub iss: String,
    /// Unique id for the JWT, used for identification within the blacklist
    pub jti: String,
    pub scopes: Vec<String>,
    pub(crate) email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenDetails {
    pub token: String,
    pub token_uuid: Uuid,
    pub user_id: String,
    pub expires_in: i64,
    pub scopes: Vec<String>,
}
