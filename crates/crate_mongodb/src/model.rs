
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, Document, serde_helpers::{serialize_bson_datetime_as_rfc3339_string, serialize_hex_string_as_object_id}},
    options::{ClientOptions, FindOptions},
    Client,
};
//use chrono::Utc;
use bson::DateTime;
use rand::Rng;
// https://github.com/wpcodevo/rust-axum-mongodb/blob/master/Cargo.toml

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub createdAt: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub updatedAt: DateTime,
}

pub fn create_random_item() -> Item {
    let mut rng = rand::thread_rng();
    let titles = vec!["Title 1", "Title 2", "Title 3"];
    let categories = vec![Some("Category 1".to_string()), Some("Category 2".to_string()), None];
    let published = vec![Some(true), Some(false), None];

    Item {
        id: ObjectId::new(),
        title: titles[rng.gen_range(0..titles.len())].to_string(),
        category: categories[rng.gen_range(0..categories.len())].clone(),
        published: published[rng.gen_range(0..published.len())],
        createdAt: DateTime::now(),
        updatedAt: DateTime::now(),
    }
}