// @generated automatically by Diesel CLI.

diesel::table! {
    myuser (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}
