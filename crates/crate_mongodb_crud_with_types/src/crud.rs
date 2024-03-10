use mongodb::{Client, Collection, error::Result};
use mongodb::bson::Document;
use serde::Serialize;
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use std::str::FromStr;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

use crate::model::{ItemModelFromDatabase, ItemModelToDatabase};

pub async fn get_connection(connection_string: String) -> Client {
    return Client::with_uri_str(connection_string).await.unwrap()
}
pub async fn create<T>(collection: &Collection<T>, item: &T) -> Result<String> {
    let result = collection.insert_one<T>(item, None).await?;
    
    // return id as string
    Ok(result.inserted_id.as_object_id().unwrap().to_hex())
}
  
// pub async fn read<ItemModelFromDatabase>(collection: &Collection<ItemModelFromDatabase>, filter: Document) -> Result<Vec<ItemModelFromDatabase>> {
//     let mut cursor = collection.find(filter, None).await?;
//     let mut results = Vec::new();
//     while let Some(result) = cursor.next().await {
//         results.push(result?);
//     }
//     Ok(results)
// }