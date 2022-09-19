use crate::models::db::Pool;
use actix_web::{web, Result};
use diesel::PgConnection;
use log::debug;

use crate::{auth::*, db::users::insert_user, errors::ServiceError, models::db::User};
use shared::auth::{UserLogin, UserLoginResponse};
use shared::models::NewUser;

pub async fn login(
    user_login: web::Json<UserLogin>,
) -> Result<web::Json<UserLoginResponse>, ServiceError> {
    debug!(
        "login function called for User: {:#?}",
        &user_login.username
    );
    let permissions = Vec::from(["ADMIN_ROLE".to_string()]);
    let token_str = create_token(user_login.username.clone(), permissions).await?;

    let response = UserLoginResponse {
        username: user_login.username.clone(),
        token: token_str.clone(),
    };
    Ok(web::Json(response))
}

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
