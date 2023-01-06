use crate::models::json::FormSchema;

pub fn build_form_query(form: &FormSchema, table_name: String) -> String {
    let mut query = format!("CREATE TABLE {table_name} (id SERIAL PRIMARY KEY, created_at TIMESTAMP NOT NULL DEFAULT NOW(), submitted_by integer NOT NULL REFERENCES users (id)");
    let properties = form.schema["properties"].as_object().unwrap();

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
