//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```
use axum::{
    response::{IntoResponse, Response},
    http::{self, StatusCode},
    body::Body,
    Router,
    routing::get
};
use schema::osb_github_logfiles::org_repo;

pub mod pg_db;
pub mod schema;
use crate::pg_db::{
    establish_connection, 
    execute_custom_query
};

pub async fn handler() -> impl IntoResponse {
    println!("handler");

    let mut connection = establish_connection();
    println!("connection");

    let org_repo_name = "azure-samples/azure-typescript-e2e-apps";

    let repo_data = execute_custom_query(&mut connection, &org_repo_name);
    println!("repo_data");

    // Serialize the vector of users as JSON.
    let json_data = match serde_json::to_string(&repo_data) {
        Ok(json) => json,
        Err(_) => "[]".to_string(),
    };

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(json_data))
        .unwrap()
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