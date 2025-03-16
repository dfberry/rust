use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryableByName;
use diesel::RunQueryDsl;
use diesel::sql_query;
use diesel::sql_types::{Text, Timestamp, Integer};

use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use tracing::info;

#[derive(QueryableByName, Queryable, Serialize, Deserialize, Debug, Default)]
pub struct CustomQueryResult {
    #[diesel(sql_type = Integer)]
    disk_usage: i32,

    #[diesel(sql_type = Integer)]
    open_prs_count: i32,
    #[diesel(sql_type = Integer)]
    open_issues_count: i32,

    #[diesel(sql_type = Integer)]
    forks_count: i32,
    #[diesel(sql_type = Integer)]
    watchers_count: i32,
    #[diesel(sql_type = Integer)]
    stargazers_count: i32,

    #[diesel(sql_type = Timestamp)]
    created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    updated_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pushed_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    log_time: NaiveDateTime,
}

pub async fn execute_custom_query(connection: &mut PgConnection, org_repo: &str) -> Vec<CustomQueryResult> {
    let query_statement = r#"
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
            (logfile ->> 'diskUsage')::int as "disk_usage",
            (logfile ->> 'openPRsCount')::int as "open_prs_count",
            (logfile ->> 'openIssuesCount')::int as "open_issues_count",
            (logfile ->> 'forksCount')::int as "forks_count",
            (logfile ->> 'watchersCount')::int as "watchers_count",
            (logfile ->> 'stargazersCount')::int as "stargazers_count",
            (logfile ->> 'createdAt')::timestamp as "created_at",
            (logfile ->> 'updatedAt')::timestamp as "updated_at",
            (logfile ->> 'pushedAt')::timestamp as "pushed_at",
            created_at as "log_time"
        FROM T
        ORDER BY created_at DESC;
    "#;
    let query_results = sql_query(query_statement)
        .bind::<Text, _>(org_repo)
        .load::<CustomQueryResult>(connection)
        .expect("Error executing custom query");

    info!("queryResults: {:?}", query_results);

    query_results
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    info!("database_url: {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|err| panic!("Error connecting to {}: {}", database_url, err))
}