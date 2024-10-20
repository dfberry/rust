use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;

use diesel::deserialize::{self, FromSqlRow};
use dotenvy::dotenv;
use std::env;

#[derive(QueryableByName, Debug)]
struct Table {
    #[diesel(sql_type = diesel::sql_types::Text)]
    table_name: String,
}

#[derive(QueryableByName, Debug)]
struct Column {
    #[diesel(sql_type = diesel::sql_types::Text)]
    column_name: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    data_type: String,
}
#[derive(QueryableByName, Queryable, Debug)]
struct User {
    #[diesel(sql_type = diesel::sql_types::Text)]
    id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    github_id: String,
    #[diesel(sql_type = diesel::sql_types::Text)]
    username: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    println!("DATABASE_URL: {}", database_url);

    let mut connection = PgConnection::establish(&database_url)?;

    // Execute a raw SQL query to get all tables
    let tables = sql_query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
        .load::<Table>(&mut connection)?;

    for table in tables {
        println!("table_name: {}", table.table_name);
    }

    // Execute a raw SQL query to get all columns and their types from the user table
    let columns = sql_query("SELECT column_name, data_type FROM information_schema.columns WHERE table_name = 'user'")
        .load::<Column>(&mut connection)?;

    for column in columns {
        println!("column_name: {}, data_type: {}", column.column_name, column.data_type);
    }

    // Query the users table
    let results = sql_query(r#"SELECT id, github_id, username FROM "user""#)
    .load::<User>(&mut connection)?;

    // Print the results
    for user in results {
        println!("id: {}, username: {}, github_id: {}", user.id, user.username, user.github_id);
    }

    Ok(())
}