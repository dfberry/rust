use super::database_collection_item::ItemModel;
use super::request::CreateItemSchema;
use super::response::ItemResponse;
use mongodb::bson::{doc, oid::ObjectId, Document};
use chrono::prelude::*;
use std::error::Error;
use serde_json::Error as SerdeError;
use serde::Deserialize;
use crate::models::serialization_error::MyJsonSerializationError;

use super::serialization_error::MySerializationError;

pub fn database_to_response(item: ItemModel) -> Result<ItemResponse, Box<dyn Error>> {
    let item_response = ItemResponse {
        id: item.id.to_hex(),
        name: item.name.to_owned(),
        category: item.category.to_owned(),
        completedAt: item.completedAt,
        createdAt: item.createdAt,
        updatedAt: item.updatedAt,
    };

    Ok(item_response)
}

// CreateItemSchema
pub fn document_to_new_item_request(
    request: &Document
) -> Result<CreateItemSchema, MyJsonSerializationError> {


    // convert document to CreateItemSchema
    let name = match request.get_str("name") {
        Ok(name) => name.to_string(),
        Err(_) => return Err(MyJsonSerializationError::new("name field is missing or not a string")),
    };

    let category = match request.get_str("category") {
        Ok(category) => Some(category.to_string()),
        Err(_) => None,
    };

    let item = CreateItemSchema {
        name,
        category,
    };

    Ok(item)

}
// CreateItemSchema
/*
pub fn json_to_new_item_request(
    request: &str
) -> Result<CreateItemSchema, MySerializationError> {

    // convert json to Document
    let serialized_data = bson::to_bson(request)
        .map_err(|_| MySerializationError::SerializationError("Failed to serialize request".to_string()))?;

    let document = serialized_data.as_document().unwrap();

    let datetime = Utc::now();

    let mut bson_doc_with_dates = doc! {
        "createdAt": datetime,
        "updatedAt": datetime
    };
    bson_doc_with_dates.extend(document.clone());

    Ok(bson_doc_with_dates)
}
*/