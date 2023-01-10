use chrono::NaiveDateTime;
use serde::Serialize;
use crate::database::schema::users;
use crate::models::json::RegisterUserSchema;
use crate::utils::{serialize_timestamp};

#[derive(Debug, Queryable, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub validated: bool,
    pub admin: bool,
    pub form_ids: Vec<Option<i32>>,
    #[serde(serialize_with = "serialize_timestamp")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "serialize_timestamp")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl<'a> From<&'a RegisterUserSchema> for NewUser<'a> {
    fn from(schema: &'a RegisterUserSchema) -> Self {
        NewUser {
            username: &schema.username,
            password: &schema.password,
            email: &schema.email,
        }
    }
}
