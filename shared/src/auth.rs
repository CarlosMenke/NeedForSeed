use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub username: String,
    pub token: String,
}
