use crate::models::db::Pool;
use actix_web::{web, Result};
use diesel::PgConnection;
use log::debug;

use crate::{auth::*, db::users::insert_user, errors::ServiceError, models::db::User};
use shared::auth::{UserPermissions, UserPermissionsResponse};
use shared::models::NewUser;

pub async fn login(
    user_permissions: web::Json<UserPermissions>,
) -> Result<web::Json<UserPermissionsResponse>, ServiceError> {
    debug!(
        "login function called for User: {:#?} with Permisstions: {:#?}",
        &user_permissions.username, &user_permissions.permissions
    );
    let token_str = create_token(
        user_permissions.username.clone(),
        user_permissions.permissions.clone(),
    )
    .await?;

    let response = UserPermissionsResponse {
        username: user_permissions.username.clone(),
        permissions: user_permissions.permissions.clone(),
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
