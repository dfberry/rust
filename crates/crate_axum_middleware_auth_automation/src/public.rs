use axum::{
    routing::get,
    Router,
};

pub fn public_routes() -> Router {
    Router::new()
        .route("/", get(public))
        .route("/public", get(public))
}

#[axum::debug_handler]
async fn public() -> &'static str {
    "Public"
}