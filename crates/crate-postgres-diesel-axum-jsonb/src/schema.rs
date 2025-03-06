// @generated automatically by Diesel CLI.

diesel::table! {
    osb_github_logfiles (id) {
        id -> Uuid,
        org_repo -> Text,
        logfile -> Jsonb,
        created_at -> Timestamptz,
    }
}
