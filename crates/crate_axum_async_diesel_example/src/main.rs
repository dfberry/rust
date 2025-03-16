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
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use diesel::sql_query;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde::Serialize;
use dotenvy::dotenv;
use std::env;


type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;


#[derive(QueryableByName, Serialize, Debug)]
pub struct Table {
    #[diesel(sql_type = diesel::sql_types::Text)]
    table_name: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", db_url);

    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    let pool = bb8::Pool::builder().build(config).await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/tables", get(list_tables))
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn list_tables(
    State(pool): State<Pool>
) -> Result<Json<Vec<Table>>, (StatusCode, String)> {

    println!("Listing tables in the database");

    // ERROR: Timeout happens here
    /*
    thread 'tokio-runtime-worker' panicked at crates/axum-async-diesel-example/src/main.rs:79:37:
    called `Result::unwrap()` on an `Err` value: TimedOut
    Listing tables in the database
     */
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