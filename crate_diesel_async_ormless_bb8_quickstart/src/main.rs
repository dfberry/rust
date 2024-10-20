//! Run with
//!
//! ```sh
//! export DATABASE_URL=postgres://localhost/your_db
//! diesel migration run
//! cargo run -p example-diesel-async-postgres
//! ```
//!
//! Checkout the [diesel webpage](https://diesel.rs) for
//! longer guides about diesel
//!
//! Checkout the [crates.io source code](https://github.com/rust-lang/crates.io/)
//! for a real world application using axum and diesel

use axum::{
    extract::{State},
    http::{StatusCode},
    response::Json,
    routing::get,
    Router,
};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use serde::Serialize;
use std::net::SocketAddr;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use diesel::sql_query;
use serde_json::json;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(QueryableByName, Serialize, Debug)]
pub struct Table {
    #[diesel(sql_type = diesel::sql_types::Text)]
    table_name: String,
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);

    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/", get(hello_world_handler))
        .route("/tables", get(list_tables))
        .with_state(pool);
    
    let addr = format!("0.0.0.0:{}", 3005);
    println!("Address: {}", addr);

    // run it with hyper
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world_handler() -> Json<serde_json::Value> {
    Json(json!({"message": "Hello, World!"}))
}

async fn list_tables(
    State(pool): State<Pool>
) -> Result<Json<Vec<Table>>, (StatusCode, String)> {

    println!("Listing tables in the database");

    let mut conn = pool.get().await.unwrap();
    println!("Connection pool checked out");

    let tables: Vec<Table> = sql_query(r#"SELECT table_name FROM information_schema.tables WHERE table_schema='public'"#)
        .load(&mut conn)
        .await
        .expect("Error loading tables");

    println!("Print tables in the database:");
    for table in &tables {
        println!("{}", table.table_name);
    }

    Ok(Json(tables))
}