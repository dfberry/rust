use diesel::prelude::*;
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel::{Queryable, backend::Backend};
use serde_json::Value;
use std::io::Write;
use diesel::Insertable;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

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
pub struct NewLogFile<'a> {
    #[diesel(sql_type = diesel::sql_types::Jsonb)]
    pub logfile: &'a serde_json::Value,
    pub org_repo: &'a str,
}