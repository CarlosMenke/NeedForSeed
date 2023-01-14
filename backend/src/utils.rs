use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::*;
use glob::glob;
use log::debug;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::iter::zip;
use std::path::Path;

use crate::errors::ServiceError;

pub const FILE_DIR: &str = "./files";
pub const PATH_TIME_SPEND: &str = "time_spend.dat";
// all finance files. First one is the default
pub const PATH_FINANCE_FILES: [&'static str; 4] =
    ["gesamt.dat", "nachhilfe.dat", "invest.dat", "rent.dat"];
// the display names of finance files. They are matched by index with the PATH_FINANCE_FILES.
pub const NAME_FINANCE: [&'static str; 4] = ["Gesamt", "Nachhilfe", "Invest", "Wohnung"];

///Hashes password with the same settings that are used in data table
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

///Verifies Password hash
pub fn verify(password_hash: &str, password: &str) -> Result<bool, ServiceError> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

//TODO just get suggestion for one target die (timeManagment or so)
/// get html file suggestions
pub fn html_suggestion(user: &str) -> Result<Vec<shared::models::HtmlSuggestion>, ServiceError> {
    let mut content_html = Vec::new();
    let files = format!("./files/{}/*/*.html", &user);

    let get_date = Regex::new(r"-.*$").unwrap();
    let get_depth = Regex::new(r"^.*-").unwrap();
    let get_category = Regex::new(r"^.*/").unwrap();

    for path in glob(&files).unwrap().filter_map(Result::ok) {
        let file = Path::new(&path.display().to_string())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let date = get_date.replace(&file, "").to_string();
        let depth = get_depth.replace(&file, "").to_string();
        let category = get_category
            .replace(
                &path
                    .display()
                    .to_string()
                    .replace(&format!("/{}.html", file), "")
                    .to_string(),
                "",
            )
            .to_string();
        let content = shared::models::HtmlSuggestion {
            target: category,
            timespan: file
                .replace(&format!("{}-", &date), "")
                .replace(&format!("-{}", &depth), ""),
            date,
            depth,
        };
        content_html.push(content);
    }
    Ok(content_html)
}

/// converts the ledger file for Time tracking and extracts the Heandline and the target account
pub fn ledger_time_suggestion(
    user: &str,
) -> Result<Vec<shared::models::TimeEnterySuggestion>, ServiceError> {
    let mut suggestion = Vec::new();

    let ledger = fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))?;
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
            if headline == "" {
                debug!("No headline{:?}", line);
            }
            let account_target = remove_first_tab
                .replace_all(&remove_time.replace(line, "").to_string(), "")
                .to_string();
            suggestion.push(shared::models::TimeEnterySuggestion {
                headline: headline.clone(),
                account_target,
            });
        } else {
            pos += 1;
        }
    }
    Ok(suggestion)
}

/// get the n last time enteries
pub fn ledger_history(
    user: &str,
    target: shared::models::HistoryTargetFile,
) -> Result<Vec<shared::models::EnteryHistory>, ServiceError> {
    let path = match target {
        shared::models::HistoryTargetFile::TimeManagment => PATH_TIME_SPEND,
        shared::models::HistoryTargetFile::Finance => PATH_FINANCE_FILES[0],
    };
    let mut history = Vec::new();
    let ledger = fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, path))?;
    let mut pos: i32 = 0; //log line number of entery
    let mut headline: String = "".to_string(); //temp store of headline
    let mut date: String = "".to_string(); //temp store of date
    let mut timespan: String = "".to_string(); //temp store of timespan
    let mut remove_entery: String = "".to_string(); //temp store of timespan
    let mut duration = 0;

    //checks if the line is the beginning if a new entery
    let check_beginning = Regex::new(r"^\d{4}/\d{2}/\d{2}").unwrap();
    let check_timespan = Regex::new(r"^; \d{2}:\d{2} - \d{2}:\d{2}").unwrap();
    let replace_date = Regex::new(r"^\d{4}/\d{2}/\d{2}[ ]*[\t]*[ ]*").unwrap();
    let get_timespan = Regex::new(r"^; ").unwrap();
    let get_date = Regex::new(r"^\d{4}/\d{2}/\d{2}").unwrap();
    let get_duration = Regex::new(r"[-]*\d{1,4}[m, h]+").unwrap();
    let remove_time = Regex::new(r"[\s]*[\t]*\d{1, 3}[\.]?\d{0,2}[m,h]").unwrap();
    let remove_first_tab = Regex::new(r"[\s]*\t").unwrap();
    let mut tracking: bool = false;
    for line in ledger.lines() {
        //TODO only date date, if it is one line befor headline.
        if check_timespan.is_match(line) {
            remove_entery = format!("\n{}\n", line);
            timespan = get_timespan.replace(line, "").to_string();
        }
        if check_beginning.is_match(line) {
            remove_entery += &format!("{}\n", line);
            pos = 0;
            tracking = true;
            date = match get_date.find(&line) {
                Some(t) => t.as_str().to_string(),
                _ => "0000/00/00".to_string(),
            };
            headline = replace_date.replace(line, "").to_string();
        } else if pos == 0 && tracking {
            remove_entery += &format!("{}\n", line);
            pos += 1;
            match get_duration.find(&line) {
                Some(e) => duration = e.as_str().replace("m", "").parse::<i32>().unwrap_or(0),
                None => (),
            };
        } else if pos == 1 && tracking {
            pos = 0;
            tracking = false;
            if headline == "" {
                debug!("No headline{:?}", line);
            }
            remove_entery += &format!("{}", line);
            let account_target = remove_first_tab
                .replace_all(&remove_time.replace(line, "").to_string(), "")
                .to_string();
            match get_duration.find(&line) {
                Some(e) => duration = e.as_str().replace("m", "").parse::<i32>().unwrap_or(0),
                None => (),
            };
            history.push(shared::models::EnteryHistory {
                remove_entery: remove_entery.clone(),
                date: date.clone(),
                timespan: timespan.clone(),
                headline: headline.clone(),
                account_target,
                duration,
            });
            remove_entery = "".to_string();
        } else {
            pos += 1;
        }
    }
    Ok(history)
}

/// Starts time Entery in ledger time File.
pub fn ledger_start_time_entery(
    user: &str,
    start_entery: shared::models::StartTimeEntery,
) -> Result<String, ServiceError> {
    println!("{:?}", start_entery.headline);
    let dt = chrono::Local::now();
    let minutes_count = (i64::from(dt.hour() * 60 + dt.minute())
        + i64::from(start_entery.offset.unwrap_or(0))
        + 24 * 60)
        % (24 * 60);
    let chrono_date = chrono::Local::now();
    let date = format!(
        "{:?}/{:02}/{:02}",
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
    println!("{}", format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND));
    fs::OpenOptions::new()
        .append(true)
        .open(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))?
        .write_all(format!("{}\n", entery).as_bytes())?;
    return Ok(entery.to_string());
}

///Remove started time File
pub fn ledger_kill_time_entery(user: &str, remove_line: String) -> Result<String, ServiceError> {
    let ledger = fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))?;
    fs::File::create(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))
        .unwrap()
        .write(
            ledger
                .replace(&format!("{}\n", &remove_line), "")
                .as_bytes(),
        )?;
    Ok(remove_line)
}

/// It returns all found started enterys in the ledger file for time_spend.
pub fn ledger_get_running_time_entery(
    user: &str,
) -> Result<BTreeMap<String, shared::models::NewTimeEntery>, ServiceError> {
    let mut response = BTreeMap::new();
    let ledger = fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))?;
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
    user: &str,
    info: &shared::models::StopLedgerTimeEntery,
) -> Result<(), ServiceError> {
    let ledger = fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))?;
    fs::File::create(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))?.write(
        ledger
            .replace(&format!("{}\n", &info.remove_line), "")
            .as_bytes(),
    )?;
    let mut create_entery = info.new_entery.clone();
    create_entery.duration =
        (create_entery.duration as i32 + create_entery.offset.unwrap_or(0)) as u32;
    ledger_create_time_entery(&user, create_entery)?;
    Ok(())
}

///Creates a new time Entery
pub fn ledger_create_time_entery(
    user: &str,
    start_entery: shared::models::NewTimeEntery,
) -> Result<String, ServiceError> {
    let offset_end = start_entery.offset.unwrap_or(0);
    debug!("OFFSET END: {}", offset_end);
    let chrono_date = chrono::Local::now();
    let stop_minute: i64 =
        (i64::from(chrono::Local::now().hour() * 60 + chrono::Local::now().minute())
            + offset_end as i64
            + 24 * 60)
            % (24 * 60);
    debug!("STOP MIN END: {}", stop_minute);
    let date_now = format!(
        "{:?}/{:02}/{:02}",
        chrono_date.year(),
        chrono_date.month(),
        chrono_date.day()
    );
    let start_minute: i64 = (stop_minute - start_entery.duration as i64 + 24 * 60) % (24 * 60); //TODO adjust
    let time_span = format!(
        "; {:02}:{:02} - {:02}:{:02}",
        start_minute / 60,
        start_minute % 60,
        stop_minute / 60,
        stop_minute % 60
    );
    let date = match &start_entery.date {
        Some(d) => d,
        None => &date_now,
    };

    // calculate number of tabs
    let tab_count = if (start_entery.account_target.chars().count() / 4) < 11 {
        start_entery.account_target.chars().count() / 4
    } else {
        10
    };
    let tabs = "\t".repeat(11 - tab_count);
    let entery = &format!(
        "\n{}\n{}\t\t\t{}\n \t{}\n \t{}{}{}m\n",
        time_span,
        date,
        start_entery.headline,
        start_entery.account_origin,
        start_entery.account_target,
        tabs,
        start_entery.duration,
    );
    fs::OpenOptions::new()
        .append(true)
        .open(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))?
        .write_all(entery.as_bytes())?;
    Ok(entery.to_string())
}

pub fn ledger_create_finance_entery(
    user: &str,
    new_entery: shared::models::NewFinanceEntery,
) -> Result<String, ServiceError> {
    let pos = NAME_FINANCE
        .iter()
        .position(|f| f.to_string() == new_entery.target_file)
        .unwrap_or(0);
    let path = PATH_FINANCE_FILES[pos];
    let chrono_date = chrono::Local::now();
    let date_now = format!(
        "{:?}/{:02}/{:02}",
        chrono_date.year(),
        chrono_date.month(),
        chrono_date.day()
    );
    let date = match &new_entery.date {
        Some(d) => d,
        None => &date_now,
    };
    // calculate number of tabs
    let tab_count = if (new_entery.account_target.chars().count() / 4) < 11 {
        new_entery.account_target.chars().count() / 4
    } else {
        1
    };
    let tabs = "\t".repeat(11 - tab_count);

    let entery = &format!(
        "\n{}\t\t\t{}\n \t{}\n \t{}{}{}€\n",
        date,
        &new_entery.headline,
        &new_entery.account_origin,
        &new_entery.account_target,
        tabs,
        &new_entery.ammount,
    );
    fs::OpenOptions::new()
        .append(true)
        .open(format!("{}/{}/{}", FILE_DIR, &user, path))?
        .write_all(entery.as_bytes())?;
    Ok(entery.to_string())
}

/// converts the ledger file for Finance tracking and extracts the Heandline and ammount, target
/// and origin account
//TODO sort by date
pub fn ledger_finance_suggestion(
    user: &str,
) -> Result<Vec<shared::models::NewFinanceEntery>, ServiceError> {
    let mut content_finance = Vec::new();

    let mut pos: i32 = 0; //log line number of entery
    let mut headline: String = "".to_string(); //temp store of headline
    let mut account_origin: String = "".to_string(); //temp store of headline

    //checks if the line is the beginning if a new entery
    let check_beginning = Regex::new(r"^\d{4}/\d{2}/\d{2}").unwrap();
    let replace_date = Regex::new(r"^\d{4}/\d{2}/\d{2}[ ]*[\t]*[\s]*").unwrap();
    let get_account = Regex::new(r"[\s, \t]*(-)?\d{1, 4}[\.]?\d{0,2}€").unwrap();
    let remove_first_tab = Regex::new(r"^[\s, \t]*").unwrap();
    let remove_last_tab = Regex::new(r"[\s, \t]*$").unwrap();
    let get_ammount = Regex::new(r"[-]*\d{0,6}[.]*\d{1,6}€").unwrap();
    let mut tracking: bool = false;
    let mut ammount = 0.0;
    let mut content;

    for (file, target_file) in zip(PATH_FINANCE_FILES, NAME_FINANCE) {
        let ledger = fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, file))?;
        //TODO add multi line enteryies
        for line in ledger.lines() {
            if check_beginning.is_match(line) {
                pos = 0;
                tracking = true;
                headline = replace_date.replace(line, "").to_string();
            } else if pos == 0 && tracking {
                account_origin = remove_last_tab
                    .replace(
                        &remove_first_tab
                            .replace(&get_account.replace(line, "").to_string(), "")
                            .to_string(),
                        "",
                    )
                    .to_string();
                pos += 1;
                match get_ammount.find(&line) {
                    Some(e) => ammount = e.as_str().replace("€", "").parse::<f32>().unwrap_or(0.0),
                    None => (),
                };
            } else if pos == 1 && tracking {
                match get_ammount.find(&line) {
                    Some(e) => ammount = e.as_str().replace("€", "").parse::<f32>().unwrap_or(0.0),
                    None => (),
                };
                pos = 0;
                tracking = false;
                content = shared::models::NewFinanceEntery {
                    headline: headline.clone(),
                    account_target: remove_last_tab
                        .replace(
                            &remove_first_tab
                                .replace(&get_account.replace(line, "").to_string(), "")
                                .to_string(),
                            "",
                        )
                        .to_string(),
                    account_origin: account_origin.clone(),
                    date: None,
                    ammount,
                    target_file: target_file.to_string(),
                };
                //check if entery exists in vec
                //TODO dont push, if just ammount is different
                if !content_finance.contains(&content) {
                    content_finance.push(content);
                }
            } else {
                pos += 1;
            }
        }
    }
    Ok(content_finance)
}

#[cfg(test)]
mod tests {
    use shared::models::NewTimeEntery;

    use super::*;

    pub const TEST_USER: &str = "test";

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
        assert!(ledger_create_time_entery(&TEST_USER, new_entery.clone()).is_ok());

        //remove added line
        let ledger =
            fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &TEST_USER, PATH_TIME_SPEND)).unwrap();
        fs::File::create(format!("{}/{}/{}", FILE_DIR, &TEST_USER, PATH_TIME_SPEND))
            .unwrap()
            .write(
                ledger
                    .replace(
                        &ledger_create_time_entery(&TEST_USER, new_entery).unwrap(),
                        "",
                    )
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
        let remove_line = ledger_start_time_entery(&TEST_USER, start_entery).unwrap();
        println!("{}", &remove_line);
        //TODO find error
        assert!(ledger_get_running_time_entery(&TEST_USER)
            .unwrap()
            .get(&remove_line)
            .is_some());

        //remove added line
        let ledger =
            fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &TEST_USER, PATH_TIME_SPEND)).unwrap();
        fs::File::create(format!("{}/{}/{}", FILE_DIR, &TEST_USER, PATH_TIME_SPEND))
            .unwrap()
            .write(
                ledger
                    .replace(&format!("{}\n", &remove_line), "")
                    .as_bytes(),
            )
            .unwrap();
    }

    #[actix_web::test]
    async fn test_ledger_kill_time_entery() {
        let start_entery = shared::models::StartTimeEntery {
            headline: "Carlos is programming".to_owned(),
            account_origin: "FreeTime".to_owned(),
            account_target: "EducationRust".to_owned(),
            duration: None,
            date: None,
            offset: None,
        };
        let remove_line = ledger_start_time_entery(&TEST_USER, start_entery).unwrap();
        //TODO find error
        assert!(ledger_kill_time_entery(&TEST_USER, remove_line).is_ok());
    }

    #[actix_web::test]
    async fn test_ledger_create_finance_entery() {
        let mut new_entery = shared::models::NewFinanceEntery {
            headline: "Carlos is programming".to_owned(),
            account_origin: "FreeTime".to_owned(),
            account_target: "Girokonto:N2".to_owned(),
            ammount: 10 as f32,
            date: None,
            target_file: "Finance".to_string(),
        };
        let mut remove_line = ledger_create_finance_entery(&TEST_USER, new_entery.clone()).unwrap();
        for _i in 1..1 {
            new_entery.account_target += "6";
            remove_line = ledger_create_finance_entery(&TEST_USER, new_entery.clone()).unwrap();
        }
        //TODO find error
        assert!(ledger_kill_time_entery(&TEST_USER, remove_line.clone()).is_ok());
        //remove added line
        let ledger = fs::read_to_string(format!(
            "{}/{}/{}",
            FILE_DIR, &TEST_USER, PATH_FINANCE_FILES[0]
        ))
        .unwrap();
        fs::File::create(format!(
            "{}/{}/{}",
            FILE_DIR, &TEST_USER, PATH_FINANCE_FILES[0]
        ))
        .unwrap()
        .write(ledger.replace(&format!("{}", &remove_line), "").as_bytes())
        .unwrap();
    }

    #[actix_web::test]
    async fn test_ledger_suggestion_finance_entery() {
        let suggestion = ledger_finance_suggestion(&TEST_USER);
        //println!("{:#?}", suggestion.as_ref().unwrap());
        for ent in suggestion.as_ref().unwrap() {
            if ent.account_target.contains(char::is_whitespace) {
                println!("{:#?}", ent);
            }
        }
        assert!(suggestion.is_ok());
    }

    #[actix_web::test]
    async fn test_ledger_history_time_entery() {
        let suggestion =
            ledger_history(&TEST_USER, shared::models::HistoryTargetFile::TimeManagment);
        println!("{:#?}", suggestion.as_ref().unwrap());
        assert!(suggestion.is_ok());
    }

    #[actix_web::test]
    async fn test_html_suggestion() {
        let suggestion = html_suggestion(&"Carlos");
        println!("{:#?}", suggestion.as_ref().unwrap());
        assert!(suggestion.is_ok());
    }
}
