use diesel::prelude::*;
use diesel::sql_types::{Text, Timestamp};
use diesel::table;

table! {
    users (id) {
        id -> Text,
        github_id -> Text,
        username -> Text,
    }
}
