use diesel::RunQueryDsl;
use crate::database::DbConnection;
use crate::database::models::{CreateUserDTO, UserDTO};
use crate::models::json::RegisterUserSchema;

impl DbConnection {
    pub fn register_user(&mut self, new_user: RegisterUserSchema) -> Result<(), warp::Rejection> {
        use crate::database::schema::users;

        let result = diesel::insert_into(users::table)
            .values(&CreateUserDTO::from(&new_user))
            .get_result::<UserDTO>(&mut self.connection)
            .map_err(|error| {
                warp::reject::reject()
            });
        println!("Result: {:?}", result);
        Ok(())
    }
}
