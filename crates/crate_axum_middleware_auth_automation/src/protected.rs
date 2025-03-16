use axum::{
    http::{StatusCode, Request},
    response::Response,
    routing::get,
    Router,
    middleware::Next,
};

pub fn protected_routes() -> Router {
    Router::new()
        .route("/protected", get(protected).route_layer(axum::middleware::from_fn(admin_key_middleware)))
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