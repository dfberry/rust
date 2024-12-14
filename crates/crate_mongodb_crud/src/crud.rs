use mongodb::{Client, Collection, error::Result};
use mongodb::bson::Document;
use serde::Serialize;
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use std::str::FromStr;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;

pub async fn get_connection(connection_string: String) -> Client {
    return Client::with_uri_str(connection_string).await.unwrap()
}


pub async fn create<T: Serialize + Send + Sync>(collection: &Collection<T>, item: &T) -> Result<String> {
    let result = collection.insert_one(item).await?;
    Ok(result.inserted_id.as_object_id().unwrap().to_hex())
}

pub async fn read2<'a, T: DeserializeOwned + Send + Sync + 'a>(
    collection: &Collection<T>,
    filter: Document,
    sort: Option<Document>,
    limit: Option<i64>,
) -> Result<Vec<T>> {


    let doc_sort: Document = match sort {
        Some(sort) => sort,
        None => doc! { "name": -1 },
    };

    let doc_limit: i64 = match limit {
        Some(limit) => limit,
        None => 100,
    };

    let mut cursor = collection
    .find(filter)
    .sort(doc_sort)
    .limit(doc_limit)
    .await?;

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        results.push(result?);
    }
    Ok(results)
}
pub async fn read<'a, T: DeserializeOwned + Send + Sync + 'a>(collection: &Collection<T>, filter: Document) -> Result<Vec<T>> {
    let mut cursor = collection.find(filter).await?;
    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        results.push(result?);
    }
    Ok(results)
}

pub async fn read_by_id<T: DeserializeOwned + Send + Sync + Unpin>(collection: &Collection<T>, id: &str) -> Result<Option<T>> {
    let object_id = ObjectId::from_str(id).unwrap();
    let filter = doc! { "_id": object_id };
    collection.find_one(filter).await
}

pub async fn update<T: Serialize + Send + Sync>(collection: &Collection<T>, filter: Document, update: Document) -> Result<()> {
    let update_result = collection.update_one(filter, update,).await?;
    if update_result.modified_count == 1 {
        Ok(())
    } else {
        Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "No document was updated")))
    }
}

pub async fn update_by_id<T: Serialize + Send + Sync>(collection: &Collection<T>, id: &str, update_document: Document) -> Result<()> {
    let object_id = ObjectId::from_str(id).unwrap();
    let filter = doc! { "_id": object_id };
    let update_result = collection.update_one(filter, update_document).await?;
    if update_result.modified_count == 1 {
        Ok(())
    } else {
        Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "No document was updated")))
    }
}

pub async fn delete<T: Serialize + Send + Sync>(collection: &Collection<T>, filter: Document) -> Result<()> {
    let delete_result = collection.delete_one(filter).await?;
    if delete_result.deleted_count == 1 {
        Ok(())
    } else {
        Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "No document was deleted")))
    }
}

pub async fn delete_by_id<T: Serialize + Send + Sync>(collection: &Collection<T>, id: &str) -> Result<()> {
    let object_id = ObjectId::from_str(id).unwrap();
    let filter = doc! { "_id": object_id };
    let delete_result = collection.delete_one(filter).await?;
    if delete_result.deleted_count == 1 {
        Ok(())
    } else {
        Err(mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::Other, "No document was deleted")))
    }
}