use jsonschema::{JSONSchema};
use serde::Deserialize;
use serde_json::Value;

// @TODO: make this list easier to maintain
const FORBIDDEN_KEYS: [&str; 3] = ["id", "submitted_by", "created_at"];
const ALLOWED_TYPES: [&str; 3] = ["string", "number", "boolean"];

#[derive(Deserialize)]
pub struct FormSchema {
    pub name: String,
    pub description: String,
    pub questions: Vec<String>,
    // later converted to json schema
    pub schema: Value,
}

impl FormSchema {
    pub fn validate(&self) -> Result<(), String> {
        // check if there is type field set to 'object'
        if self.schema.get("type").is_none() ||
            self.schema.get("type").unwrap() != "object" {
            return Err("Schema must have a type set to object".to_string());
        }
        // check if there is properties field which is an object and is not empty
        let properties = self.schema.get("properties");
        if properties.is_none() || !properties.unwrap().is_object() {
            return Err("Schema must have an object properties".to_string());
        }
        // check if properties length is equal to questions length
        if properties.unwrap().as_object().unwrap().len() != self.questions.len() {
            return Err("Schema properties length must be equal to questions length".to_string());
        }
        // check if properties is not empty
        if properties.unwrap().as_object().unwrap().is_empty() {
            return Err("Schema must have at least one field".to_string());
        }
        // loop in properties and check for forbidden fields (used by db for context tracking) or types|
        for (key, value) in properties.unwrap().as_object().unwrap() {
            if FORBIDDEN_KEYS.contains(&key.as_str()) {
                return Err(format!("Field {key} is forbidden"));
            }
            if value.get("type").is_none() ||
                !ALLOWED_TYPES.contains(&value.get("type").unwrap().as_str().unwrap()) {
                return Err(format!("Field {key} must have a type set to string, number or boolean"));
            }
        }
        // we then try to compile the schema for any other errors
        match JSONSchema::compile(&self.schema) {
            Err(error) => Err(error.to_string()),
            Ok(_) => Ok(()),
        }
    }

    pub fn build_form_query(&self, table_name: String) -> String {
        let mut query = format!("CREATE TABLE {table_name} (id SERIAL PRIMARY KEY, created_at TIMESTAMP NOT NULL DEFAULT NOW(), submitted_by integer NOT NULL REFERENCES users (id)");
        let properties = self.schema["properties"].as_object().unwrap();

        for (name, value) in properties {
            let column_type = match value["type"].as_str() {
                Some("string") => "TEXT",
                Some("number") => "INTEGER",
                Some("boolean") => "BOOLEAN",
                _ => "TEXT",
            };
            query.push_str(&format!(", {name} {column_type}"));
        }
        query.push(')');
        query
    }
}
