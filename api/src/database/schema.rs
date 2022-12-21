// @generated automatically by Diesel CLI.

diesel::table! {
    forms (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        jsonschema -> Jsonb,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
        validated -> Bool,
        admin -> Bool,
        form_ids -> Array<Nullable<Int4>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(forms -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    forms,
    users,
);
