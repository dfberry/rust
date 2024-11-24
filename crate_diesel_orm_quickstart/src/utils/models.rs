use diesel::prelude::*;
use chrono::DateTime;
use chrono::Utc;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::utils::schema::github_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GitHub_User {
    pub id: String,
    pub github_id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}