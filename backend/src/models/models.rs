use super::schema::users;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub registerd_time: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
