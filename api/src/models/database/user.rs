use chrono::NaiveDateTime;
use crate::database::schema::users;
use crate::models::json::RegisterUserSchema;

#[derive(Debug, Queryable, Clone)]
pub struct UserDTO {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserDTO<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl<'a> From<&'a RegisterUserSchema> for CreateUserDTO<'a> {
    fn from(schema: &'a RegisterUserSchema) -> Self {
        CreateUserDTO {
            username: &schema.username,
            password: &schema.password,
            email: &schema.email,
        }
    }
}
