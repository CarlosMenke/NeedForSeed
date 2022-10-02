use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseBTreeMap {
    pub map: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct NewTimeEntery {
    pub headline: String,
    pub account_origin: String,
    pub account_target: String,
}
