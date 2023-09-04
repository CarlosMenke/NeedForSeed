use crate::models::schema::users;
use diesel::{r2d2, r2d2::ConnectionManager, SqliteConnection};
use diesel::{Insertable, Queryable};
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_id: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
