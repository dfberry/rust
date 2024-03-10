use axum::{
    routing::get,
    Router,
    response::{IntoResponse, Response},
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(get_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

use mongodb::{bson::doc, bson::oid::ObjectId, bson::Document, options::ClientOptions, options::FindOptions, Client, Collection, results::DeleteResult, results::UpdateResult};
use mongodb::bson::serde_helpers;

use serde::Serialize;
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use std::str::FromStr;

async fn get_handler() -> impl IntoResponse {
    
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name =
        std::env::var("MONGODB_DATABASE_NAME").expect("MONGODB_DATABASE_NAME must be set.");
    let collection_name =
        std::env::var("MONGODB_COLLECTION_NAME").expect("MONGODB_COLLECTION_NAME must be set.");
    (connection_string, database_name, collection_name);

    let client_options = ClientOptions::parse(&connection_string).await?;
    let client = Client::with_options(client_options)?;

    let items: Vec<Document> = client.database(&database_name).collection("items").find(None, None).await?;

    Json(items)
}
