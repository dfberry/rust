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
    extract::{State, FromRequest, Request},
    http::{StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response, Json},
    routing::get,
    Router,
};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection, RunQueryDsl,
};
use diesel_async::pooled_connection::deadpool::Pool;
use serde::Serialize;
use std::net::SocketAddr;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use diesel::sql_query;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);
    
    // set up connection pool
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();

    // build our application with some routes
    let app = Router::new()
        .route("/", get(root_get_handler))
        .route("/tables", get(list_tables))
        .with_state(pool)
        .layer(middleware::from_fn(print_request_body));

    let addr = format!("0.0.0.0:{}", 3005);
    println!("Address: {}", addr);

    // run it with hyper
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
async fn print_request_body(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    let method = request.method().clone();
    let uri = request.uri().clone();

    println!("Requested route: {} {}", method, uri);

    Ok(next.run(request).await)
}
pub async fn root_get_handler() -> Html<String> {
    let html_content = format!(
        "<h1>Source board</h1>"
    );

    Html(html_content)
}
async fn list_tables(
    State(pool): State<Pool<AsyncPgConnection>>
) -> Result<Json<Vec<Table>>, (StatusCode, String)> {

    println!("Starting list_tables");

    // checkout a connection from the pool
    let mut conn = pool.get().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("Checked out connection from pool");

    let tables: Vec<Table> = sql_query(r#"SELECT table_name FROM information_schema.tables WHERE table_schema='public'"#)
        .load(&mut conn)
        .await
        .expect("Error loading tables");

        println!("Tables in the database:");
    for table in &tables {
        println!("{}", table.table_name);
    }

    Ok(Json(tables))
}