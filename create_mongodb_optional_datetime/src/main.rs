
use dotenv::dotenv;
use mongodb::Collection;
use mongodb::Client;
use mongodb::bson::doc;
use rand::Rng;
use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use bson::serde_helpers::serialize_object_id_as_hex_string;
use bson::serde_helpers::deserialize_hex_string_from_object_id;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemModel {
    pub _id: ObjectId,
    pub name: String,
    pub category: String,
    pub completedAt: Option<DateTime<Utc>>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}
// pub struct ItemModel {
//     #[serde(serialize_with = "serialize_hex_string_as_object_id")]
//     pub _id: ObjectId,
//     pub name: String,
//     pub category: String,
//     pub completedAt: Option<DateTime<Utc>>,
//     pub createdAt: DateTime<Utc>,
//     pub updatedAt: DateTime<Utc>,
// }

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv().ok();
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = "Test";
    let collection_name = "Items";
    
    let client = Client::with_uri_str(connection_string).await.unwrap();
    let database = client.database(database_name);
    let collection_item_model = database.collection::<ItemModel>(collection_name);

    // insert one item
    let item = ItemModel {
        _id: ObjectId::new(),
        name: "Item 1".to_string(),
        // add random number to category
        category: "Category1".to_string() + &rand::thread_rng().gen_range(0..100).to_string(),
        completedAt: None,
        createdAt: Utc::now(),
        updatedAt: Utc::now(),
    };

    // insert item in database
    let result = collection_item_model.insert_one(item, None).await.unwrap();

    // get id from result
    let id = result.inserted_id.as_object_id().unwrap();

    println!("Inserted item with id: {}", id);

    // read item from database
    let item = collection_item_model.find_one(doc! { "_id": id }, None).await.unwrap().unwrap();

    // convert item to json
    let json = bson::to_bson(&item).unwrap();

    // print json
    println!("{}", json);


}
