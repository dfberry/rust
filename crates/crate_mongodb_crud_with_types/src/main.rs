use dotenv::dotenv;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use mongodb::{Collection, Client};
use mongodb::bson::{doc, self, oid::ObjectId};
use rand::Rng;
use futures::TryStreamExt;
use std::str::FromStr;
use std::error::Error;
use chrono::prelude::*;
use serde_with::{serde_as, DisplayFromStr, Map};
use crate::bson::Bson;
#[serde_as]
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewModelToDatabase {
    pub name: String,
    pub category: String, // shard key for cosmos db
    pub completedAt: Option<DateTime<Utc>>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}
use bson::serde_helpers::serialize_object_id_as_hex_string;
#[serde_as]
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExistingModel {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub _id: ObjectId,
    pub name: String,
    pub category: String, // shard key for cosmos db
    pub completedAt: Option<DateTime<Utc>>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[serde_as]
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonModel {
    pub id: String,
    pub name: String,
    pub category: String, // shard key for cosmos db
    pub completedAt: Option<DateTime<Utc>>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Item {
//     name: String,
//     category: String
// }

#[tokio::main]
async fn main()-> Result<Vec<Value>, Box<dyn Error>>{

    dotenv().ok();
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = "Test";
    let collection_name = "Items";
    
    let client = Client::with_uri_str(connection_string).await?;
    let collection: Collection<NewModelToDatabase> = client.database(database_name).collection(collection_name);
    let collection2: Collection<Value> = client.database(database_name).collection(collection_name);

    // create randmom number
    let mut rng = rand::thread_rng();
    let random_number: String = rng.gen_range(0..100).to_string().parse().unwrap();

    let name = format!("Serde types {}", random_number);
    let category:String = "cat".to_string();

    // Add - for name
    let new_item: NewModelToDatabase = NewModelToDatabase{
        name: name.clone(),
        category: category.clone(),
        completedAt: None,
        createdAt: Utc::now(),
        updatedAt: Utc::now(),
    };
    let result = collection.insert_one(new_item, None).await?;
    
    // print all fields of inserted document
    println!("Inserted document with id: {}", result.inserted_id.as_object_id().unwrap().to_hex());
    println!("Item: {:?}", result);


    let id = result.inserted_id.as_object_id().unwrap().to_hex();

    // find doc by id
    let object_id = ObjectId::from_str(&id).unwrap();
    let filter = doc! { "_id": object_id };
    let item = collection.find_one(filter, None).await?;
    println!("Found item: {:?}", item);
 
    // get all from collection
    let mut cursor = collection2.find(None, None).await?;
    let mut results: Vec<Value> = Vec::new();
    while let Some(result) = cursor.try_next().await? {

            let bson: Bson = mongodb::bson::from_document(result)?;
            let json: Value = serde_json::to_value(bson)?;
            results.push(json);

    }
    
    Ok(results)

}
