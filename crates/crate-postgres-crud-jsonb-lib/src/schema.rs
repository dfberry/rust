// @generated automatically by Diesel CLI.

diesel::table! {
    logfiles (id) {
        id -> Uuid,
        logfile -> Jsonb,
        created_at -> Timestamptz,
        org_repo -> Text,
    }
}