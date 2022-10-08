use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

use crate::errors::ServiceError;

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

//TODO find better return type
pub fn ledger_start_time_entery(
    headline: &str,
    origin: &str,
    target: &str,
) -> Result<(), ServiceError> {
    println!("{:?}", headline);
    let dt = chrono::Local::now();
    let minutes_count = dt.hour() * 60 + dt.minute();
    let chrono_date = chrono::Local::now();
    let date = format!(
        "{:?}/{:?}/{:02}",
        chrono_date.year(),
        chrono_date.month(),
        chrono_date.day()
    );
    //first line
    let entery = &format!(
        ";{} {}\t\t\t {};\t {};\t {}\t\t\t\t\t\t ##m\n",
        &minutes_count.to_string(),
        &date.to_string(),
        headline,
        origin,
        target
    );

    //TODO find a way how to close the file again
    fs::OpenOptions::new()
        .append(true)
        .open("./files/time_spend.dat")
        .expect("Unable to open file")
        .write_all(entery.as_bytes())
        .expect("write failed");
    return Ok(());
}

/// It returns all found started enterys in the ledger file for time_spend.
pub fn ledger_get_running_time_entery(
) -> Result<BTreeMap<String, shared::models::NewTimeEntery>, ServiceError> {
    let mut response = BTreeMap::new();
    let ledger = fs::read_to_string("./files/time_spend.dat")
        .expect("Should have been able to read the file");
    let stop_minute: u32 = chrono::Local::now().hour() * 60 + chrono::Local::now().minute();

    let get_started_enteries = Regex::new(r"^;[0-9]").unwrap();
    let get_start_minute = Regex::new(r"[0-9]+ ").unwrap();
    let new_line = Regex::new(r";").unwrap();
    let get_content = Regex::new(r" .*").unwrap();
    for l in ledger.lines() {
        if get_started_enteries.is_match(l) {
            //TODO remove this, unessesary
            let line = l.to_string();
            let start_minute_str = get_start_minute.find(&line).unwrap().as_str();
            let content_raw = get_content.find(&line).unwrap().as_str();
            let content = new_line.replace_all(content_raw, "\n").to_string(); // replace ; with \n
            let start_minute: u32 = start_minute_str.trim().parse().unwrap();
            let mut offset = 0;
            if start_minute > stop_minute {
                offset += 60 * 24;
            }
            let duration = offset + stop_minute - start_minute;
            let time_span = format!(
                "{:02}:{:02} - {:02}:{:02}",
                start_minute / 60,
                start_minute % 60,
                stop_minute / 60,
                stop_minute % 60
            );
            let content_vec = content.split("\n").collect::<Vec<&str>>();
            let new_entery = shared::models::NewTimeEntery {
                headline: content_vec[0]
                    .to_string()
                    .split("\t")
                    .collect::<Vec<&str>>()[0]
                    .to_string(),
                account_origin: content_vec[1].to_string(),
                account_target: content_vec[2].to_string(),
                time_span,
                duration,
                date: content_vec[1]
                    .to_string()
                    .split("\t")
                    .collect::<Vec<&str>>()[0]
                    .to_string(),
            };
            response.insert(l.to_string(), new_entery);
        }
    }
    Ok(response)
}
