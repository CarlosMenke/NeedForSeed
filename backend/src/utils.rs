use crate::errors::ServiceError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

///Hashes password with the same settings that are used in data table
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

///Verifies Password hash
#[allow(dead_code)]
pub fn verify(password_hash: &str, password: &str) -> Result<bool, ServiceError> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

use regex::Regex;
use std::collections::BTreeMap;
use std::fs;

//TODO make function more generic: give the file path
/// converts the ledger file for Music tracking and extracts the Heandline and the buttom content
pub fn ledger_time_content() -> Result<BTreeMap<String, String>, ServiceError> {
    let mut content_headline = BTreeMap::new();

    let ledger = fs::read_to_string("./files/time_spend.dat")?;
    let mut pos: i32 = 0; //log line number of entery
    let mut headline: String = "".to_string(); //temp store of headline

    //checks if the line is the beginning if a new entery
    let check_beginning = Regex::new(r"^\d{4}/\d{2}/\d{2}").unwrap();
    let replace_date = Regex::new(r"^\d{4}/\d{2}/\d{2}[ ]*[\t]*[ ]*").unwrap();
    let remove_time = Regex::new(r"[\s]*[\t]*\d{1, 3}[\.]?\d{0,2}[m,h]").unwrap();
    let remove_first_tab = Regex::new(r"[\s]*\t").unwrap();
    let mut tracking: bool = false;
    for line in ledger.lines() {
        if check_beginning.is_match(line) {
            pos = 0;
            tracking = true;
            headline = replace_date.replace(line, "").to_string();
        } else if pos == 1 && tracking {
            pos = 0;
            tracking = false;
            content_headline.insert(
                remove_first_tab
                    .replace_all(&remove_time.replace(line, "").to_string(), "")
                    .to_string(),
                headline.clone(),
            );
        } else {
            pos += 1;
        }
    }
    Ok(content_headline)
}
