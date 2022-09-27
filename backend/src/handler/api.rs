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
};
use shared::auth::{UserLogin, UserLoginResponse};
use shared::models::{NewUser, ResponseHtml};

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
    let permissions = Vec::from(["ADMIN_ROLE".to_string(), "GET_INFO_MUSIC".to_string()]);
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

/// get Music history
#[has_permissions("GET_INFO_MUSIC")]
pub async fn get_music(path: web::Path<u32>) -> Result<web::Json<ResponseHtml>, ServiceError> {
    let depth = path.into_inner();
    debug!("Get Music function called for depth: {:#?}", &depth);
    match fs::read_to_string(format!("./files/music_{}.html", depth)) {
        Ok(f) => return Ok(web::Json(ResponseHtml { html: f })),
        Err(_) => {
            return Err(ServiceError::InternalServerError(
                "Unable to read file".to_string(),
            ))
        }
    };
}
