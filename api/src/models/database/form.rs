use chrono::NaiveDateTime;
use crate::database::schema::forms;
use serde_json::Value;
use crate::models::json::FormSchema;

#[derive(Queryable)]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub schema: Value,
    pub created_by: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "forms"]
pub struct NewForm<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub jsonschema: Value,
    pub created_by: i32,
}

impl<'a> NewForm<'a> {
    pub fn from(schema: &'a FormSchema, user_id: i32) -> Self {
        NewForm {
            name: &schema.name,
            description: &schema.description,
            jsonschema: schema.schema.clone(),
            created_by: user_id,
        }
    }
}
