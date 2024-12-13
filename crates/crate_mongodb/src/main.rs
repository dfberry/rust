#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::env;

use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document, serde_helpers::{serialize_bson_datetime_as_rfc3339_string, serialize_hex_string_as_object_id}},
    options::{ClientOptions, FindOptions},
    Client,
    Collection
};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub mod model;

use model::{Item, create_random_item};

#[derive(Debug)]
struct Err {}
impl From<mongodb::error::Error> for Err {
    fn from(_error: mongodb::error::Error) -> Self {
        Err {}
    }
}

#[allow(dead_code)]
type Result<T> = std::result::Result<T, Err>;

pub fn get_random_string() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    use rand::distributions::Alphanumeric;

    let chars: String = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect();
    println!("Random chars: {}", chars);
    chars
}

// pub fn list_all_collections_in_db(database: &mongodb::Database) -> Result<()> {
//     let collections = database.list_collection_names(None).expect("error occured");
//     for collection_name in collections {
//         println!("{}", collection_name);
//     }
//     Ok(())
// }

pub async fn insert_into_collection(collection: &mongodb::Collection<Item>, item: Item) -> Result<()> {
    let result = collection.insert_one(item, None).await.expect("error occured");
    println!("{:?}", result);
    Ok(())
}

// pub fn insert_many_into_collection(collection: &mongodb::Collection, items: Vec<Item>) -> Result<()> {
//     let result = collection.insert_many(items).expect("error occured");
//     println!("{:?}", result);
//     Ok(())
// }

// pub fn find_one(collection: &mongodb::Collection, filter: Document) -> Result<Option<Item>> {
//     let result = collection.find_one(filter, None).expect("error occured");
//     match result {
//         Some(doc) => {
//             let item: Item = bson::from_bson(bson::Bson::Document(doc)).expect("error occured");
//             Ok(Some(item))
//         }
//         None => Ok(None),
//     }
// }

// pub async fn find_many(collection: &mongodb::Collection, filter: Document) -> Result<Vec<Item>> {


//     let mut cursor = nontyped_collection.find({}).await.expect("error occured"); 
//     while let Some(doc) = cursor.try_next().await.unwrap() { 
//         println!("{:?}", doc) 
//     } 
// }


#[tokio::main]
async fn main() -> Result<()> {

    dotenv().ok();

    let db_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");

        let client_options = ClientOptions::parse(
            db_url.as_str()
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("Todo");
        let collection: Collection<Item> = database.collection("Items");

        let item = create_random_item();
        insert_into_collection(&collection, item)?;

    
        Ok(())
}
