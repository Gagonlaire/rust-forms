use chrono::{Utc};
use diesel::{Connection, QueryResult, RunQueryDsl, sql_types::Integer, sql_query, insert_into, QueryDsl};
use crate::database::{DbConnection, DbResult};
use crate::database::schema::forms::created_by;
use crate::models::database::{NewForm, Form};
use crate::models::json::FormSchema;

impl DbConnection {
    pub fn create_form(&mut self, schema: &FormSchema, user_id: i32) -> DbResult<Form> {
        DbResult::from(self.connection.transaction(|conn| {
            use crate::database::schema::forms::dsl::forms;

            let table_name = format!("table_{}", Utc::now().timestamp_nanos());
            let new_form: QueryResult<Form> = insert_into(forms)
                .values(NewForm::from(schema, user_id, &table_name))
                .get_result(conn);
            let update_query = "UPDATE users SET form_ids = array_append(form_ids, $1) WHERE id = $2";

            sql_query(update_query)
                .bind::<Integer, i32>(new_form.as_ref().unwrap().id)
                .bind::<Integer, i32>(user_id)
                .execute(conn)?;
            sql_query(schema.build_form_query(table_name))
                .execute(conn)?;

            Ok(new_form.unwrap())
        }))
    }

    pub fn get_form(&mut self, form_id: i32) -> DbResult<Form> {
        use crate::database::schema::forms::dsl::forms;

        DbResult::from(forms.find(form_id).first(&mut self.connection))
    }
}
