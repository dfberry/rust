use super::database_collection_item::ItemModel;
use super::request::CreateItemSchema;
use super::response::ItemResponse;
use mongodb::bson::{doc, oid::ObjectId, Document};
use chrono::prelude::*;
use std::error::Error;

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

pub fn request_to_bson_document(
    request: &CreateItemSchema,
    published: bool,
    category: String,
) -> Result<Document, MySerializationError> {
    let serialized_data = bson::to_bson(request)
        .map_err(|_| MySerializationError::SerializationError("Failed to serialize request".to_string()))?;
    let document = serialized_data.as_document().unwrap();

    let datetime = Utc::now();

    let mut bson_doc_with_dates = doc! {
        "createdAt": datetime,
        "updatedAt": datetime,
        "published": published,
        "category": category
    };
    bson_doc_with_dates.extend(document.clone());

    Ok(bson_doc_with_dates)
}