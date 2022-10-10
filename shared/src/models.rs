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

//TODO rename to better name, like ResponseTimeSuggestion
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseBTreeMap {
    pub map: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewTimeEntery {
    pub headline: String,
    pub account_origin: String,
    pub account_target: String,
    pub time_span: String,
    pub date: String,
    pub duration: u32,
}

//TODO should all send data start with Request as Prefix?
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartTimeEntery {
    pub headline: String,
    pub account_origin: String,
    pub account_target: String,
}
impl Default for StartTimeEntery {
    fn default() -> StartTimeEntery {
        StartTimeEntery {
            headline: String::new(),
            account_origin: "FreeTime".to_string(),
            account_target: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ResponseStatus {
    pub status: i8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseRunningLedgerTimeEntery {
    pub running_entery: BTreeMap<String, NewTimeEntery>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopLedgerTimeEntery {
    pub new_entery: NewTimeEntery,
    pub remove_line: String,
}
