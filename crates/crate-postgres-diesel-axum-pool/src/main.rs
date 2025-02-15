use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Response},
    http::{self, StatusCode},
    body::Body,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub mod schema;
use crate::schema::test_table_2;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::test_table_2)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub date_created: NaiveDateTime,
}

// Use a type alias for convenience
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_users(connection: &mut PgConnection) -> Vec<User> {
    test_table_2::table
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users")
}

pub async fn handler(pool: axum::extract::State<PgPool>) -> impl IntoResponse {
    println!("handler async");

    // Get a connection from the pool in a blocking task
    let users = {
        let pool = pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().expect("Failed to get connection");
            get_users(&mut conn)
        })
        .await
        .unwrap()
    };

    // Serialize the vector of users as JSON.
    let json_users = match serde_json::to_string(&users) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    };

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_users))
        .unwrap()
}
pub async fn get_users2(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    test_table_2::table
        .limit(5)
        .select(User::as_select())
        .load(conn)
}
pub async fn handler2(pool: axum::extract::State<PgPool>) -> impl IntoResponse {
    println!("handler async");

    // Get a connection from the pool asynchronously.
    let mut conn = pool.get().expect("Failed to get connection");
    let users = get_users2(&mut conn)
        .await
        .expect("Error loading users");

    // Serialize the vector of users as JSON.
    let json_users = serde_json::to_string(&users)
        .unwrap_or_else(|_| "[]".to_string());

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_users))
        .unwrap()
}

#[tokio::main]
async fn main() {
    // Initialize connection pool.
    let pool = establish_connection_pool();

    // Build our application with a route.
    let app = Router::new()
        .route("/", get(handler))
        .route("/async", get(handler2))
        .with_state(pool);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}