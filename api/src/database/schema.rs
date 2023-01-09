// @generated automatically by Diesel CLI.

diesel::table! {
    dummy (id) {
        id -> Int4,
        created_at -> Timestamp,
        submitted_by -> Int4,
    }
}

diesel::table! {
    forms (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        questions -> Array<Nullable<Text>>,
        jsonschema -> Jsonb,
        table_name -> Text,
        created_by -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    table_1673013827738586000 (id) {
        id -> Int4,
        created_at -> Timestamp,
        submitted_by -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    table_1673017869300954000 (id) {
        id -> Int4,
        created_at -> Timestamp,
        submitted_by -> Int4,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    test (id) {
        id -> Int4,
        created_at -> Timestamp,
        submitted_by -> Int4,
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

diesel::joinable!(dummy -> users (submitted_by));
diesel::joinable!(forms -> users (created_by));
diesel::joinable!(table_1673013827738586000 -> users (submitted_by));
diesel::joinable!(table_1673017869300954000 -> users (submitted_by));
diesel::joinable!(test -> users (submitted_by));

diesel::allow_tables_to_appear_in_same_query!(
    dummy,
    forms,
    table_1673013827738586000,
    table_1673017869300954000,
    test,
    users,
);
