use jsonschema::{JSONSchema, ValidationError};
use serde::Deserialize;
use serde_json::Value;

// @TODO: make this list easier to maintain
const FORBIDDEN_KEYS: [&str; 3] = ["id", "submitted_by", "created_at"];
const ALLOWED_TYPES: [&str; 3] = ["string", "number", "boolean"];

#[derive(Deserialize)]
pub struct FormSchema {
    pub name: String,
    pub description: String,
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
}
