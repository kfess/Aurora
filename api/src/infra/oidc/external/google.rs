use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub sub: String,
    pub at_hash: Option<String>,
    pub azp: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub hd: Option<String>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub nonce: Option<String>,
    pub picture: Option<String>,
    pub profile: Option<String>,
}
