use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, Map};

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
pub struct ItemModelToDatabase {
    pub name: String,
    pub item_type: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct ItemModelFromDatabase {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub item_type: String,
}