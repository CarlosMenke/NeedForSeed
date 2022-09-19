use actix_web::{web, Result};
use log::debug;

use crate::{auth::*, errors::ServiceError};

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
