use dotenv::dotenv;
use json::convert_vec_to_json;
use mongodb::Collection;
use mongodb::Client;
use mongodb::bson::doc;
use rand::Rng;
use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use crate::models::database_collection_item::ItemModel;
use crate::business_logic::get_all_items;

use crate::bson::Document;

pub mod models;
pub mod json;
pub mod db;
pub mod business_logic;



#[tokio::main]
async fn main() {

    dotenv().ok();
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = "Test";
    let collection_name = "Items";
    
    let client = Client::with_uri_str(connection_string).await.unwrap();
    let database = client.database(database_name);
    let collection_item_model = database.collection::<ItemModel>(collection_name);
    let collection2_document = database.collection::<Document>(collection_name);

    // create randmom number
    let mut rng = rand::thread_rng();
    let random_number: String = rng.gen_range(0..100).to_string().parse().unwrap();

    let typedlist = get_all_items(&collection_item_model).await.unwrap();
    let jsonlist = convert_vec_to_json(typedlist);
    jsonlist.iter().for_each(|item| println!("{}", item));
}