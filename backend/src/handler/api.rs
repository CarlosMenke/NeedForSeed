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
use shared::auth::{UserLogin, UserLoginResponse};
use shared::models::{NewUser, ResponseBTreeMap, ResponseHtml};

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
    let permissions = Vec::from(["ADMIN_ROLE".to_string(), "GET_LEDGER_INFO".to_string()]);
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
    path: web::Path<(String, String, String)>,
) -> Result<web::Json<ResponseHtml>, ServiceError> {
    let (target, depth, timeframe) = path.into_inner();
    debug!(
        "Get HTML function called for target: \t {:#?} \tdepth: \t{:#?} \ttimeframe: \t{:#?}",
        &target, &depth, &timeframe
    );
    //TODO make path more general. right now, it only works, if cargo run is executed one dir above
    //main.rs
    let file = fs::read_to_string(format!("./files/{}_{}_{}.html", target, depth, timeframe))?;
    Ok(web::Json(ResponseHtml { html: file }))
}

/// get Headline and Content BTreeMap from Ledger Music
#[has_permissions("GET_LEDGER_INFO")]
pub async fn get_ledger_time_suggetstions() -> Result<web::Json<ResponseBTreeMap>, ServiceError> {
    debug!("Get Ledger Time Suggestion");
    Ok(web::Json(ResponseBTreeMap {
        hash_map: utils::ledger_time_content()?,
    }))
}
