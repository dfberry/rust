use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::github_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GitHub_User {
    pub id: String,
    pub github_id: String,
    pub username: String
}