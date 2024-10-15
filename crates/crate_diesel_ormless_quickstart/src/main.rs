use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;
use dotenvy::dotenv;
use serde_json::Value;
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

#[derive(QueryableByName, Debug)]
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

    // Execute a raw SQL query to get all rows from the user table
    let users = sql_query("SELECT id, username FROM user")
        .load::<User>(&mut connection)?;

    for user in users {
        println!("id: {}, username: {}", user.id, user.username);
    }

    Ok(())
}