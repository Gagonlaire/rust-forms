use chrono::NaiveDateTime;
use serde::{Serialize};
use crate::utils::{serialize_timestamp};
use crate::database::schema::forms;
use serde_json::Value;
use crate::models::json::FormSchema;
use crate::utils::vec_to_option_vec;

#[derive(Queryable, Serialize)]
pub struct Form {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub questions: Vec<Option<String>>,
    pub jsonschema: Value,
    pub table_name: String,
    pub created_by: i32,
    #[serde(serialize_with = "serialize_timestamp")]
    pub created_at: NaiveDateTime,
    #[serde(serialize_with = "serialize_timestamp")]
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = forms)]
pub struct NewForm<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub questions: Vec<Option<String>>,
    pub jsonschema: Value,
    pub table_name: &'a str,
    pub created_by: i32,
}

impl<'a> NewForm<'a> {
    pub fn from(schema: &'a FormSchema, user_id: i32, table_name: &'a String) -> Self {
        NewForm {
            name: &schema.name,
            description: &schema.description,
            questions: vec_to_option_vec(schema.questions.clone()),
            jsonschema: schema.schema.clone(),
            table_name,
            created_by: user_id,
        }
    }
}
