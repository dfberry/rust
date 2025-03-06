//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```
use axum::{
    Json,
    response::{IntoResponse, Response},
    http::{self, StatusCode},
    body::Body,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;
use diesel::sql_query;
use diesel::sql_types::{Text, Timestamp, Jsonb};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::RunQueryDsl;

use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

use axum::{response::Html, routing::get, Router};
use rand::Rng;
use serde_json::Value;

pub mod schema;
use crate::schema::osb_github_logfiles;

#[derive(FromSqlRow, AsExpression, serde::Serialize, serde::Deserialize, Debug, Default)]
#[diesel(sql_type = Jsonb)]
struct CustomQueryResult {
    #[diesel(sql_type = Text)]
    disk_usage: Option<String>,
    #[diesel(sql_type = Text)]
    fork_count: Option<String>,
    #[diesel(sql_type = Text)]
    open_prs_count: Option<String>,
    #[diesel(sql_type = Text)]
    watches_count: Option<String>,
    #[diesel(sql_type = Text)]
    issues_count: Option<String>,
    #[diesel(sql_type = Text)]
    stars_count: Option<String>,
    #[diesel(sql_type = Timestamp)]
    created_at: chrono::NaiveDateTime,
}
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::osb_github_logfiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct Logfile {
    id: String,
    org_repo: String,
    logfile: Value,
    created_at: NaiveDateTime,
}
// Implement Queryable for serde_json::Value
impl Queryable<Jsonb, Pg> for Value {
    type Row = serde_json::Value;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row)
    }
}


// Implement FromSql for serde_json::Value
impl FromSql<Jsonb, Pg> for Value {
    fn from_sql(bytes: diesel::pg::PgValue) -> deserialize::Result<Self> {
        let bytes = bytes.as_bytes();
        serde_json::from_slice(bytes).map_err(|e| e.into())
    }
}

pub fn execute_custom_query(connection: &PgConnection) -> Vec<CustomQueryResult> {
    let query = r#"
        WITH T as (
            SELECT
                org_repo,
                logfile,
                created_at
            FROM
                public.osb_github_logfiles
            WHERE
                org_repo = 'azure-samples/azure-typescript-e2e-apps'
            ORDER BY
                created_at DESC
            LIMIT 30
        )
        SELECT
            logfile ->> 'diskUsage' as "disk_usage",
            logfile ->> 'forkCounts' as "fork_count",
            logfile ->> 'openPRsCount' as "open_prs_count",
            logfile ->> 'watchesCount' as "watches_count",
            logfile ->> 'openIssuesCount' as "issues_count",
            logfile ->> 'starsCount' as "stars_count",
            created_at
        from T
        ORDER BY created_at DESC;
    "#;
    sql_query(query)
        .load::<CustomQueryResult>(connection)
        .expect("Error executing custom query")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn handler() -> impl IntoResponse {
    println!("handler");

    let connection = establish_connection();
    println!("connection");

    let repo_data = execute_custom_query(&connection);
    println!("repo_data");

    // Serialize the vector of users as JSON.
    let json_data = match serde_json::to_string(&repo_data) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    };

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_data))
        .unwrap()
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}