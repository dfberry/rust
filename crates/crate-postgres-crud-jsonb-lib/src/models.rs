use diesel::prelude::*;
use diesel::Queryable;
use diesel::Insertable;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde_json::Value;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::logfiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Logfile {
    pub id: Uuid,
    #[diesel(sql_type = Jsonb)]
    pub logfile: Value,
    pub created_at: NaiveDateTime,
    pub org_repo: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::logfiles)]
pub struct NewLogfile<'a> {
    pub logfile: &'a Value,
    pub org_repo: &'a str,
}