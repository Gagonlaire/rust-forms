use diesel::{QueryDsl, RunQueryDsl};
use crate::database::{DbConnection, DbResult};
use crate::models::database::{CreateUserDTO, UserDTO};
use crate::models::json::RegisterUserSchema;
use crate::diesel::ExpressionMethods;
use crate::database::schema::users;

impl DbConnection {
    pub fn register_user(&mut self, new_user: RegisterUserSchema) -> DbResult<UserDTO> {
        DbResult::from(
            diesel::insert_into(users::table)
                .values(&CreateUserDTO::from(&new_user))
                .get_result(&mut self.connection)
        )
    }

    pub fn get_user_by_email(&mut self, email: &str) -> DbResult<UserDTO> {
        DbResult::from(
            users::table
                .filter(users::email.eq(email))
                .first(&mut self.connection)
        )
    }

    pub fn get_user_by_id(&mut self, id: i32) -> DbResult<UserDTO> {
        DbResult::from(
            users::table
                .filter(users::id.eq(id))
                .first(&mut self.connection)
        )
    }
}
