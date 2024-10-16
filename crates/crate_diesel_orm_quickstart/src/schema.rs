// @generated automatically by Diesel CLI.

diesel::table! {
    app_user_watch_repos (id) {
        id -> Text,
        user_id -> Text,
        repo_name -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    github_user_sessions (id) {
        id -> Text,
        user_id -> Text,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    github_user_tokens (id) {
        id -> Text,
        user_id -> Text,
        encrypted_access_token -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    github_users (id) {
        id -> Text,
        github_id -> Text,
        username -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(app_user_watch_repos -> github_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user_watch_repos,
    github_user_sessions,
    github_user_tokens,
    github_users,
);
