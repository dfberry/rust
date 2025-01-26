use dotenvy::dotenv;
use serde::Serialize;
use serde::Deserialize;
use mongodb::Collection;
use mongodb::Client;
use mongodb::bson::doc;
use rand::Rng;

pub mod crud;
pub mod model;
pub mod tests;

pub use tests::{tests_by_id, tests_by_name};
pub use model::Item;

#[tokio::main]
async fn main() {

    dotenv().ok();
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = "Test";
    let collection_name = "Items";
    
    let client = Client::with_uri_str(connection_string).await.unwrap();
    let database = client.database(database_name);
    let collection: Collection<Item> = database.collection(collection_name);

    // create randmom number
    let mut rng = rand::thread_rng();
    let random_number: String = rng.gen_range(0..100).to_string().parse().unwrap();

    tests_by_name(&random_number, &collection).await;
    tests_by_id(&random_number, &collection).await;

}