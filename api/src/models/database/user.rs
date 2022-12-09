use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use super::super::schema::users;

#[derive(Debug, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
