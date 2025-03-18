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
use diesel::Insertable;

use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

use axum::{response::Html, routing::get, Router};
use rand::Rng;

pub mod schema;
use crate::schema::test_table_2;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::test_table_2)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub date_created: NaiveDateTime
}


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_users(connection: &mut PgConnection) -> Vec<User> {

    test_table_2::table
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users")
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

pub async fn handler(
) -> impl IntoResponse {

    println!("handler");

    let mut connection = establish_connection();
    println!("connection"); 
 
    let users = get_users(&mut connection).await;
    println!("users"); 

    // Serialize the vector of users as JSON.
    let json_users = match serde_json::to_string(&users) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    };
    println!("json_users"); 

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_users))
        .unwrap()
}


