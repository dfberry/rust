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
use dotenvy::dotenv;
use serde::Serialize;
use serde::Deserialize;
use mongodb::Collection;
use mongodb::Client;
use mongodb::bson::doc;
use axum::{
    extract::{FromRef, FromRequestParts, State, Path, Extension, Query},
    http::{request::Parts, StatusCode},
    response::Json,
    routing::{get, post},
    Router
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::env;

pub mod crud;
pub mod model;

pub use model::Item;
pub use crud::*;

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
    let mongo_client = getMongoClient().await;
    let database_name = "Test";
    let collection_name = "Items";
    let database = mongo_client.database(database_name);
    let collection: Collection<Item> = database.collection(collection_name);


    // build our application with some routes
    let app = Router::new()
        .route("/list", get(list))
        .with_state(collection);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn getMongoClient() -> Client {
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = "Test";
    let collection_name = "Items";
    
    Client::with_uri_str(connection_string).await.unwrap()

}
#[derive(Deserialize)]
pub struct LogQuery {
    org: String,
    repo: String,
}
async fn list(
    State(collection): State<Collection<Item>>,
    Query(params): Query<LogQuery>,
) -> Result<Json<Vec<Item>>, StatusCode> {

    println!("Listing items in collection");

    let filter = doc! {}; // Empty filter
    let sort = doc! {}; // Empty sort
    let limit = 2; // Default limit

    let items: Vec<Item> = read2(&collection, filter, None, None).await.unwrap();

    println!("Print items in collection:");
    for item in &items {
        println!("{}", item.id);
    }

    Ok(Json(items))
}
