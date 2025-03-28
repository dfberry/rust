// @generated automatically by Diesel CLI.
/*

CREATE TABLE test_table_optional_datetime (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    optional_date_created timestamp,
    date_created timestamp NOT NULL DEFAULT now() NOT NULL
);

*/
diesel::table! {
    test_table_optional_datetime (id) {
        id -> Uuid,
        name -> Text,
        success -> Bool,
        optional_date_created -> Nullable<Timestamptz>,
        date_created -> Timestamptz,
    }
}