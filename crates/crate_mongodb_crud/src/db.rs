
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;
use crate::models::database_collection_item::ItemModel;
use crate::models::response::ItemListResponse;
use std::error::Error;
use crate::models::database_error::MyDBError::MongoQueryError;
/*
Find notes with DBModel
Create response array but using the Doc to note function
Function returns a NoteListResponse

pub struct NoteListResponse {
    pub status: &'static str,
    pub results: usize,
    pub notes: Vec<NoteResponse>,
}

*/
pub async fn fetch_notes( typed_collection: &Collection<ItemModel>, limit: i64, page: i64) -> Result<Vec<ItemModel>, Box<dyn Error>> {
    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = typed_collection
        .find(None, find_options)
        .await
        .map_err(MongoQueryError)?;

    let mut db_result: Vec<ItemModel> = Vec::new();
    while let Some(doc) = cursor.next().await {
        match doc {
            Ok(item) => db_result.push(item),
            Err(e) => {
                println!("Error processing document: {}", e);
                continue;
            },
        }
    }

    println!("fetch_notes returns {:?}", db_result);

    Ok(db_result)
}
