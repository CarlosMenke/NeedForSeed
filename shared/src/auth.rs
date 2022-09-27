use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLoginResponse {
    pub username: String,
    pub token: String,
}
