//! Example JWT authorization/authentication.
//!
//! Run with
//!
//! ```bash
//! ADMIN_SECRET=secret cargo run 
//! ```

use axum;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod public;
mod protected;
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

    let app = public::public_routes()
        .merge(protected::protected_routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



