use diesel::{QueryResult, RunQueryDsl};
use crate::database::{DbConnection, DbResult};
use crate::models::database::{NewForm, Form};
use crate::models::json::FormSchema;
use crate::database::schema::forms::dsl::*;

impl DbConnection {
    pub fn create_form(&mut self, schema: &FormSchema, user_id: i32) -> DbResult<Form> {
        let new_form: QueryResult<Form> = diesel::insert_into(forms)
            .values(NewForm::from(schema, user_id))
            .get_result(&mut self.connection);

        if let Err(error) = new_form {
            return DbResult::from(error);
        }
        let query = format!("UPDATE users set form_ids = array_append(form_ids, {}) WHERE id = {};", new_form.as_ref().unwrap().id, user_id);
        if let Err(error) = diesel::sql_query(query).execute(&mut self.connection) {
            return DbResult::from(error);
        }

        DbResult::Ok(new_form.unwrap())
    }
}
