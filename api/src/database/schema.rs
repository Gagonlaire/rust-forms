// @generated automatically by Diesel CLI.

diesel::table! {
    forms (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        json_schema -> Jsonb,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(forms -> users (created_by));

diesel::allow_tables_to_appear_in_same_query!(
    forms,
    users,
);
