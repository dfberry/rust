// src/main.rs
use axum::{
    routing::get,
    Router,
    response::IntoResponse,
};
use hyper::header::HeaderValue;
use std::net::SocketAddr;

async fn handler() -> impl IntoResponse {
    let mut response = axum::response::Response::new(axum::body::Body::from("Hello, Axum!"));
    response.headers_mut().insert("X-Version", HeaderValue::from_static("1.0.0"));
    response
}

#[tokio::main]
async fn main() {
    // Build our application with a single route
    let app = Router::new().route("/", get(handler));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}