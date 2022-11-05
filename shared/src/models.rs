use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

///LOGIN STRUCTS
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
pub struct HeadlineSuggestion {
    pub suggestions: Vec<TimeEnterySuggestion>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TimeEnterySuggestion {
    pub headline: String,
    pub account_target: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NewTimeEntery {
    pub headline: String,
    pub account_origin: String,
    pub account_target: String,
    pub duration: u32,
    pub date: Option<String>,
    pub offset: Option<i32>,
}

//TODO should all send data start with Request as Prefix?
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartTimeEntery {
    pub headline: String,
    pub account_origin: String,
    pub account_target: String,
    pub date: Option<String>,
    pub duration: Option<u32>,
    pub offset: Option<i32>,
}
impl Default for StartTimeEntery {
    fn default() -> StartTimeEntery {
        StartTimeEntery {
            headline: String::new(),
            account_origin: "FreeTime".to_string(),
            account_target: String::new(),
            date: None,
            duration: None,
            offset: None,
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

///Section with Finance Enterys
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NewFinanceEntery {
    pub headline: String,
    pub account_origin: String,
    pub account_target: String,
    pub ammount: f32,
    pub date: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FinanceEnterySuggestion {
    pub suggestions: Vec<NewFinanceEntery>,
}
