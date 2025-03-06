use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use serde_json::json;

use std::env;
use uuid::Uuid;
use serde_json::value;
use tracing::Value;
pub mod schema;
pub mod models;

use self::models::*;
use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

struct New_Logfile {
    logfile: Value,
    org_repo: String,
}
struct Logfile {
    id: Uuid,
    logfile: Value,
    org_repo: String,
    created_at: NaiveDateTime,
}

struct DataItem {
    pub key1: String,
    pub key2: String,
    pub key3: String,
}

fn main() {
    use self::schema::logfiles::dsl::*;

    let connection = &mut establish_connection();

    // create fake jsonb data
    let jsonb_data = json!({
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
    });

    let Logfile = New_Logfile {
        logfile: &jsonb_data,
        org_repo: "azure-samples/azure-typescript-e2e-apps",
    };

    // insert logfile into db
    diesel::insert_into(logfiles)
        .values(&Logfile)
        .execute(connection)
        .expect("Error saving new post");


    let results: Vec<Logfile> = logfiles
        .filter(org_repo.eq("azure-samples/azure-typescript-e2e-apps"))
        .order(created_at.desc())
        .limit(1)
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} logfiles", results.len());
    for repo in results {
        println!("{}", repo.id);
        println!("{}", repo.logfile);
        println!("-----------\n");
    }
}