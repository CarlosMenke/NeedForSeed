use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::*;
use log::debug;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

use crate::errors::ServiceError;

pub const PATH_TIME_SPEND: &str = "./files/time_spend.dat";

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

    let ledger = fs::read_to_string(PATH_TIME_SPEND)?;
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
    start_entery: shared::models::StartTimeEntery,
) -> Result<String, ServiceError> {
    println!("{:?}", start_entery.headline);
    let dt = chrono::Local::now();
    let minutes_count =
        i64::from(dt.hour() * 60 + dt.minute()) - i64::from(start_entery.offset.unwrap_or(0));
    let chrono_date = chrono::Local::now();
    let date = format!(
        "{:?}/{:?}/{:02}",
        chrono_date.year(),
        chrono_date.month(),
        chrono_date.day()
    );

    let entery = &format!(
        ";{} {}\t\t\t{}; \t{}; \t{}\t\t\t\t\t\t##m",
        &minutes_count.to_string(),
        &date.to_string(),
        start_entery.headline,
        start_entery.account_origin,
        start_entery.account_target,
    );

    //TODO find a way how to close the file again
    fs::OpenOptions::new()
        .append(true)
        .open(PATH_TIME_SPEND)?
        .write_all(format!("{}\n", entery).as_bytes())?;
    return Ok(entery.to_string());
}

/// It returns all found started enterys in the ledger file for time_spend.
pub fn ledger_get_running_time_entery(
) -> Result<BTreeMap<String, shared::models::NewTimeEntery>, ServiceError> {
    let mut response = BTreeMap::new();
    let ledger = fs::read_to_string(PATH_TIME_SPEND)?;
    let stop_minute: u32 = chrono::Local::now().hour() * 60 + chrono::Local::now().minute();

    let get_started_enteries = Regex::new(r"^;[0-9]").unwrap();
    let get_start_minute = Regex::new(r"[0-9]+ ").unwrap();
    let new_line = Regex::new(r";").unwrap();
    let get_content = Regex::new(r"\d{4}.*").unwrap();
    let clean_account_origin = Regex::new(r"^ \t").unwrap();
    let clean_account_target = Regex::new(r"[ ]*[\t]+[ ,#,m]*").unwrap();
    let get_date = Regex::new(r"\d{4}/\d{2}/\d{2}").unwrap();
    for line in ledger.lines() {
        if get_started_enteries.is_match(line) {
            let start_minute_str = get_start_minute.find(&line).unwrap().as_str();
            let content_raw = get_content.find(&line).unwrap().as_str();
            let content = new_line.replace_all(content_raw, "\n").to_string(); // replace ; with \n
            let start_minute: u32 = start_minute_str.trim().parse().unwrap();
            let mut offset = 0;
            if start_minute > stop_minute {
                offset += 60 * 24;
            }
            let duration = offset + stop_minute - start_minute;
            let content_vec = content.split("\n").collect::<Vec<&str>>();
            println!("vec {:?}", content_vec);
            let new_entery = shared::models::NewTimeEntery {
                headline: content_vec[0]
                    .to_string()
                    .split("\t")
                    .collect::<Vec<&str>>()[3]
                    .to_string(),
                account_origin: clean_account_origin
                    .replace(&content_vec[1].to_string(), "")
                    .to_string(),
                account_target: clean_account_target
                    .replace_all(&content_vec[2].to_string(), "")
                    .to_string(),
                duration,
                date: Some(get_date.find(&content_vec[0]).unwrap().as_str().to_string()),
                offset: None,
            };
            response.insert(line.to_string(), new_entery);
        }
    }
    debug!("Found running Enteries: {:#?}", response);
    Ok(response)
}

/// This function create a new time entery and removes the given line.
pub fn ledger_stop_time_entery(
    info: &shared::models::StopLedgerTimeEntery,
) -> Result<(), ServiceError> {
    let ledger = fs::read_to_string(PATH_TIME_SPEND)?;
    fs::File::create(PATH_TIME_SPEND)?.write(
        ledger
            .replace(&format!("{}\n", &info.remove_line), "")
            .as_bytes(),
    )?;
    ledger_create_time_entery(info.new_entery.clone())?;
    Ok(())
}

///Creates a new time Entery
pub fn ledger_create_time_entery(
    start_entery: shared::models::NewTimeEntery,
) -> Result<String, ServiceError> {
    let chrono_date = chrono::Local::now();
    let stop_minute: u32 = chrono::Local::now().hour() * 60 + chrono::Local::now().minute();
    let date_now = format!(
        "{:?}/{:?}/{:02}",
        chrono_date.year(),
        chrono_date.month(),
        chrono_date.day()
    );
    let start_minute: u32 = stop_minute - start_entery.duration; //TODO adjust
    let time_span = format!(
        "{:02}:{:02} - {:02}:{:02}",
        start_minute / 60,
        start_minute % 60,
        stop_minute / 60,
        stop_minute % 60
    );
    let date = match &start_entery.date {
        Some(d) => d,
        None => &date_now,
    };

    let mut offset = 0;
    if start_entery.duration > stop_minute {
        offset += 60 * 24;
    }
    let duration = match &start_entery.offset {
        Some(o) => i64::from(start_entery.duration) + i64::from(*o) + offset,
        None => start_entery.duration as i64,
    };

    let entery = &format!(
        "\n{}\n{}\t\t\t{}\n \t{}\n \t{}\t\t\t\t\t\t\t{}m\n",
        time_span,
        date,
        start_entery.headline,
        start_entery.account_origin,
        start_entery.account_target,
        duration,
    );
    fs::OpenOptions::new()
        .append(true)
        .open(PATH_TIME_SPEND)?
        .write_all(entery.as_bytes())?;
    Ok(entery.to_string())
}

#[cfg(test)]
mod tests {
    use shared::models::NewTimeEntery;

    use super::*;
    #[actix_web::test]
    async fn test_password_hash_and_verify() {
        let pwd = "jkl";
        let pwd_hash = &hash_password(pwd).unwrap();
        assert!(verify(pwd_hash, pwd).unwrap());
    }

    #[actix_web::test]
    async fn test_ledger_create_time_entery() {
        let new_entery = NewTimeEntery {
            headline: "Carlos is programming".to_owned(),
            account_origin: "FreeTime".to_owned(),
            account_target: "EducationRust".to_owned(),
            duration: 10,
            date: Some("2022/10/10".to_string()),
            offset: None,
        };
        assert!(ledger_create_time_entery(new_entery.clone()).is_ok());

        //remove added line
        let ledger = fs::read_to_string(PATH_TIME_SPEND).unwrap();
        fs::File::create(PATH_TIME_SPEND)
            .unwrap()
            .write(
                ledger
                    .replace(&ledger_create_time_entery(new_entery).unwrap(), "")
                    .as_bytes(),
            )
            .unwrap();
    }

    #[actix_web::test]
    async fn test_ledger_stop_time_entery() {
        let start_entery = shared::models::StartTimeEntery {
            headline: "Carlos is programming".to_owned(),
            account_origin: "FreeTime".to_owned(),
            account_target: "EducationRust".to_owned(),
            duration: None,
            date: None,
            offset: None,
        };
        let remove_line = ledger_start_time_entery(start_entery).unwrap();
        //TODO find error
        assert!(ledger_get_running_time_entery()
            .unwrap()
            .get(&remove_line)
            .is_some());

        //remove added line
        let ledger = fs::read_to_string(PATH_TIME_SPEND).unwrap();
        fs::File::create(PATH_TIME_SPEND)
            .unwrap()
            .write(
                ledger
                    .replace(&format!("{}\n", &remove_line), "")
                    .as_bytes(),
            )
            .unwrap();
    }
}
