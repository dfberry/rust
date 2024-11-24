//! Run with
//!
//! ```not_rust
//! cargo run -p example-consume-body-in-extractor-or-middleware
//! ```

use axum::{
    async_trait,
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::{response, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{post, get},
    Router,
};
use http_body_util::BodyExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use hyper::header::HeaderValue;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_consume_body_in_extractor_or_middleware=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(handler))
        .layer(middleware::from_fn(version_header));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn version_header(
    request: Request, 
    next: Next
) -> Response {
    let mut response = next.run(request).await;

    // do something with `response`...
    response.headers_mut().insert("X-Version", HeaderValue::from_static("1.0.0"));

    response
}
async fn handler() -> impl IntoResponse {
    Response::new(Body::from("Hello, Axum!"))
}
