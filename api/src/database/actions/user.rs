use diesel::{PgConnection, QueryResult, RunQueryDsl};
use crate::models::database::NewUser;

pub fn save_new_user(conn: &mut PgConnection, user: &NewUser) -> QueryResult<usize> {
    use crate::models::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(user)
        .execute(conn)
}
