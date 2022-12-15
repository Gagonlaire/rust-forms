use chrono::NaiveDateTime;
use crate::database::schema::users;

#[derive(Debug, Queryable, Clone)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
