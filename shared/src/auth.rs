use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserPermissions {
    pub username: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserPermissionsResponse {
    pub username: String,
    pub permissions: Vec<String>,
    pub token: String,
}
