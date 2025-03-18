// @generated automatically by Diesel CLI.

diesel::table! {
    test_table_2 (id) {
        id -> Uuid,
        name -> Text,
        date_created -> Timestamptz,
    }
}