use diesel::{QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use log::error;
use crate::database::DbConnection;
use crate::database::models::{CreateUserDTO, UserDTO};
use crate::database::schema::users::dsl::users;
use crate::rejections::ApiReject;
use crate::models::json::RegisterUserSchema;
use crate::replies::ApiReply;
use crate::diesel::ExpressionMethods;

impl DbConnection {
    pub fn register_user(&mut self, new_user: RegisterUserSchema) -> Result<UserDTO, ApiReject> {
        use crate::database::schema::users;

        diesel::insert_into(users::table)
            .values(&CreateUserDTO::from(&new_user))
            .get_result(&mut self.connection)
            .map_err(|error| {
                match error {
                    diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                        ApiReject::bad_request("Email already taken", None)
                    }
                    _ => {
                        error!("Error while inserting user: {}", error);
                        ApiReject::internal_error()
                    }
                }
            })
    }

    pub fn get_user_by_email(&mut self, email: &str) -> Result<UserDTO, ApiReject> {
        use crate::database::schema::users;

        users::table
            .filter(users::email.eq(email))
            .first(&mut self.connection)
            .map_err(|error| {
                if error == diesel::result::Error::NotFound {
                    ApiReject::unauthorized("Invalid credentials", None)
                } else {
                    error!("Error while getting user by email: {}", error);
                    ApiReject::internal_error()
                }
            })
    }
}
