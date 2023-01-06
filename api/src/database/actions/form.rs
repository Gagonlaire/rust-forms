use chrono::{Utc};
use diesel::{QueryResult, RunQueryDsl};
use crate::database::{DbConnection, DbResult};
use crate::models::database::{NewForm, Form};
use crate::models::json::FormSchema;
use crate::database::schema::forms::dsl::*;

impl DbConnection {
    pub fn create_form(&mut self, schema: &FormSchema, user_id: i32) -> DbResult<Form> {
        let _table_name = format!("table_{}", Utc::now().timestamp_nanos());
        let new_form: QueryResult<Form> = diesel::insert_into(forms)
            .values(NewForm::from(schema, user_id, &_table_name))
            .get_result(&mut self.connection);

        if let Err(error) = new_form {
            return DbResult::from(error);
        }
        let query = format!("UPDATE users set form_ids = array_append(form_ids, {}) WHERE id = {};", new_form.as_ref().unwrap().id, user_id);
        if let Err(error) = diesel::sql_query(query).execute(&mut self.connection) {
            return DbResult::from(error);
        }
        let query = schema.build_form_query(_table_name);
        if let Err(error) = diesel::sql_query(query).execute(&mut self.connection) {
            return DbResult::from(error);
        }

        DbResult::Ok(new_form.unwrap())
    }
}
