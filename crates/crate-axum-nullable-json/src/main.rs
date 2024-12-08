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
use serde::Deserialize;
use serde_json::json;
use axum::{response::Html, routing::get, Router};
use rand::Rng;

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

    let user: User = generate_nullable_data();

    let returned_json = json!({
        "id": user.id,
        "name": user.name
    });

    println!("Returned JSON: {}", returned_json.to_string());

    Response::builder()
        .header(http::header::CONTENT_TYPE, "application/json")
        .status(StatusCode::OK)
        .body(Body::from(returned_json.to_string()))
        .unwrap()
}
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>
}

pub fn generate_nullable_data() -> User {
    User {
        id: "1".to_string(),
        name: None
    }
}