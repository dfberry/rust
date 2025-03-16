//! Example JWT authorization/authentication.
//!
//! Run with
//!
//! ```bash
//! ADMIN_SECRET=secret cargo run 
//! ```

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get},
    Json, RequestPartsExt, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{
    middleware::Next,
    http::{Request}
};
// Quick instructions
//
//
// - visit the protected area using the authorized token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'x-admin-key:secret' \
//     http://localhost:3000/protected
//
// - try to visit the protected area using an invalid token
//
// curl -s \
//     -w '\n' \
//     -H 'Content-Type: application/json' \
//     -H 'x-admin-key:rat' \
//     http://localhost:3000/protected


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_jwt=debug".into()), // Adjust this to your desired default log level
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/protected", get(protected).route_layer(axum::middleware::from_fn(admin_key_middleware)))
        .route("/public", get(protected))
        .route("/", get(public));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
#[axum::debug_handler]
async fn public() -> &'static str {
    "Public"
}

#[axum::debug_handler]
async fn protected() -> &'static str {
    // Send the protected data to the user
    "Protected"
}

async fn admin_key_middleware(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    let admin_key = std::env::var("ADMIN_SECRET").expect("ADMIN_SECRET must be set");
    println!("admin_key: {}", admin_key);

    if let Some(header_value) = req.headers().get("x-admin-key") {
        println!("header_value: {:?}", &header_value);
        if header_value == admin_key.as_str() {
            println!("values match");
            return Ok(next.run(req).await);
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}

