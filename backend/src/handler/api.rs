use actix_web::{web, Result};
use actix_web_grants::proc_macro::has_permissions;
use diesel::PgConnection;
use log::debug;

use std::fs;

use crate::{
    auth::*,
    db::users::{check_login, insert_user},
    errors::ServiceError,
    models::db::{Pool, User},
    utils,
};
use shared::auth::*;
use shared::models::*;

/// Handles user Login and returns JWT
pub async fn login(
    pool: web::Data<Pool>,
    user_login: web::Json<UserLogin>,
) -> Result<web::Json<UserLoginResponse>, ServiceError> {
    debug!(
        "login function called for User: {:#?}",
        &user_login.username
    );
    let connection: &mut PgConnection = &mut pool.get().unwrap();
    if !check_login(connection, &user_login.username, &user_login.password)? {
        return Err(ServiceError::Unauthorized);
    };
    let permissions = Vec::from([
        "ADMIN_ROLE".to_string(),
        "GET_LEDGER_INFO".to_string(),
        "SET_LEDGER_INFO".to_string(),
    ]);
    let token_str = create_token(user_login.username.clone(), permissions).await?;

    let response = UserLoginResponse {
        username: user_login.username.clone(),
        token: token_str.clone(),
    };
    Ok(web::Json(response))
}

/// interface to create new user
pub async fn create_user(
    pool: web::Data<Pool>,
    user_data: web::Json<NewUser>,
) -> Result<web::Json<User>, ServiceError> {
    let connection: &mut PgConnection = &mut pool.get().unwrap();
    match insert_user(connection, &user_data.username, &user_data.password) {
        Ok(u) => return Ok(web::Json(u)),
        Err(e) => return Err(e),
    };
}

/// get Html files
#[has_permissions("GET_LEDGER_INFO")]
pub async fn get_html(
    path: web::Path<(String, String, String, String)>,
) -> Result<web::Json<ResponseHtml>, ServiceError> {
    let (target, depth, timeframe, timepoint) = path.into_inner();
    debug!(
        "Get HTML function called for target: \t {:#?} \tdepth: \t{:#?} \ttimeframe: \t{:#?}\ttimepoint: \t{:#?}",
        &target, &depth, &timeframe, &timepoint
    );
    //TODO make path more general. right now, it only works, if cargo run is executed one dir above
    //main.rs
    let file = fs::read_to_string(format!(
        "./files/{}_{}_{}_{}.html",
        target, depth, timeframe, timepoint
    ))?;
    Ok(web::Json(ResponseHtml { html: file }))
}

/// LEDGER TIME INTERACTION ///
/// get Headline and Content BTreeMap from Ledger Music
#[has_permissions("GET_LEDGER_INFO")]
//TODO make the function more generic in the future
pub async fn get_time_suggetstions() -> Result<web::Json<HeadlineSuggestion>, ServiceError> {
    debug!("Get Ledger Time Suggestion.");
    Ok(web::Json(HeadlineSuggestion {
        suggestions: utils::ledger_time_suggestion()?,
    }))
}

/// create new entery for time Tracking
#[has_permissions("SET_LEDGER_INFO")]
//TODO think of better return type
pub async fn set_time_entery_start(
    new_time_entery: web::Json<StartTimeEntery>,
) -> Result<web::Json<ResponseStatus>, ServiceError> {
    debug!(
        "Set ledger time function is called with Headline: \t{:?}\t account_origin: \t{:?}\t account_origin: \t{:?}\t duration: \t{:?}\t offset: \t{:?}",
        &new_time_entery.headline, &new_time_entery.account_origin, &new_time_entery.account_target, &new_time_entery.duration, &new_time_entery.offset);

    if &new_time_entery.account_origin == "" {
        return Err(ServiceError::BadRequest(
            "Empty account_origin provided.".to_string(),
        ));
    } else if &new_time_entery.account_target == "" {
        return Err(ServiceError::BadRequest(
            "Empty account_target provided.".to_string(),
        ));
    }
    if new_time_entery.duration.is_none() {
        //start running entery, because it has not ended yet.
        utils::ledger_start_time_entery(new_time_entery.to_owned())?;
    } else {
        //if duration is given, create the time entery.
        utils::ledger_create_time_entery(shared::models::NewTimeEntery {
            headline: String::from(&new_time_entery.headline),
            account_origin: String::from(&new_time_entery.account_origin),
            account_target: String::from(&new_time_entery.account_target),
            duration: new_time_entery.duration.unwrap(),
            date: new_time_entery.date.to_owned(),
            offset: new_time_entery.offset,
        })?;
    };
    Ok(web::Json(ResponseStatus { status: 0 }))
}

/// get all running time Enteries
#[has_permissions("GET_LEDGER_INFO")]
pub async fn get_time_entery_running(
) -> Result<web::Json<ResponseRunningLedgerTimeEntery>, ServiceError> {
    debug!("Get all Running Time Enteries.");
    return Ok(web::Json(ResponseRunningLedgerTimeEntery {
        running_entery: utils::ledger_get_running_time_entery()?,
    }));
}

/// stoping Time entery
#[has_permissions("SET_LEDGER_INFO")]
pub async fn set_time_entery_stop(
    payload: web::Json<StopLedgerTimeEntery>,
) -> Result<web::Json<ResponseStatus>, ServiceError> {
    debug!("Stop running Time Entery {:#?}", payload.new_entery);
    utils::ledger_stop_time_entery(&payload)?;
    return Ok(web::Json(ResponseStatus { status: 0 }));
}

/// kill Time entery
#[has_permissions("SET_LEDGER_INFO")]
pub async fn set_time_entery_kill(
    payload: web::Json<StopLedgerTimeEntery>,
) -> Result<web::Json<ResponseStatus>, ServiceError> {
    debug!("Kill running Time Entery {:#?}", payload.new_entery);
    utils::ledger_kill_time_entery(payload.remove_line.to_owned())?;
    return Ok(web::Json(ResponseStatus { status: 0 }));
}

/// LEDGER FINANCE INTERACTION ///
/// create ledger finance entery
#[has_permissions("SET_LEDGER_INFO")]
pub async fn set_finance_entery_create(
    payload: web::Json<NewFinanceEntery>,
) -> Result<web::Json<ResponseStatus>, ServiceError> {
    debug!("Create new Finacen Entery {:#?}", payload.to_owned());
    utils::ledger_create_finance_entery(payload.to_owned())?;
    return Ok(web::Json(ResponseStatus { status: 0 }));
}

/// get suggestions for ledger finance entery
#[has_permissions("GET_LEDGER_INFO")]
pub async fn get_finance_suggestions() -> Result<web::Json<FinanceEnterySuggestion>, ServiceError> {
    debug!("Get Ledger Finance Suggestion.");
    Ok(web::Json(FinanceEnterySuggestion {
        suggestions: utils::ledger_finance_suggestion()?,
    }))
}
