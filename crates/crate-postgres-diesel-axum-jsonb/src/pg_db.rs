use serde::{Deserialize, Serialize};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::QueryableByName;
use diesel::RunQueryDsl;
use diesel::sql_query;
use diesel::sql_types::{Text, Timestamp};

use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use tracing::info;

#[derive(QueryableByName, Serialize, Deserialize, Debug, Default)]
pub struct CustomQueryResult {
    #[diesel(sql_type = Text)]
    disk_usage: String,
    #[diesel(sql_type = Timestamp)]
    created_at: NaiveDateTime,
}

pub fn execute_custom_query(connection: &mut PgConnection) -> Vec<CustomQueryResult> {
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
            logfile ->> 'diskUsage' as "disk_usage",
            created_at
        from T
        ORDER BY created_at DESC;
    "#;
    let query_results = sql_query(query_statement)
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