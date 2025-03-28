use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde_json::json;
use std::env;

pub mod schema;
pub mod models;

use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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

    let new_record = NewLogfile {
        logfile: &jsonb_data,
        org_repo: "azure-samples/azure-typescript-e2e-apps",
    };

    // insert logfile into db
    diesel::insert_into(logfiles)
        .values(&new_record)
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