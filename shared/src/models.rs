use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseHtml {
    pub html: String,
}

// TODO test if also &str works
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseHashMap {
    pub hash_map: HashMap<String, String>,
}
