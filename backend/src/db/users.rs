use crate::errors::ServiceError;
use crate::models::db::{NewUser, User};
use crate::models::schema::users::dsl::*;
use crate::utils;
use diesel::prelude::*;
use log::{debug, info};
use uuid::Uuid;

pub fn insert_user(conn: &mut PgConnection, name: &str, pwd: &str) -> Result<User, ServiceError> {
    info!("Creating new User Name: {:?} pwd: {:?} ", &name, &pwd);

    let new_user = NewUser {
        user_id: &Uuid::new_v4().to_string(),
        username: &name,
        password: &utils::hash_password(&pwd)?,
    };

    Ok(diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error inserting new user"))
}

#[allow(dead_code)]
pub fn get_user(conn: &mut PgConnection, _username: &str) -> Result<User, String> {
    debug!("Selecting User with username: {:?}", _username);

    let mut results = users
        .filter(username.eq(_username))
        .load::<User>(conn)
        .expect("Error loading users");

    for user in results.pop() {
        return Ok(user);
    }
    return Err("UserNotFound".to_string());
}

#[allow(dead_code)]
pub fn delete_user(conn: &mut PgConnection, _username: &str) {
    info!("Delete User with username: {:?}", _username);

    diesel::delete(users.filter(username.like(_username)))
        .execute(conn)
        .expect("Error deleting posts");
}

pub fn check_login(
    conn: &mut PgConnection,
    _username: &str,
    _password: &str,
) -> Result<bool, ServiceError> {
    debug!(
        "Check Login for User with username: {:?} password:{:?}",
        &_username, &_password
    );

    let results = users
        .filter(username.eq(_username))
        .load::<User>(conn)
        .expect("Error loading users");

    for user in &results {
        if utils::verify(&user.password, &_password)? {
            info!(
                "Logging in: User {:?} Pwd:{:?} Id:{:?}",
                user.username, user.password, user.user_id
            );
            return Ok(true);
        }
    }
    return Ok(false);
}
