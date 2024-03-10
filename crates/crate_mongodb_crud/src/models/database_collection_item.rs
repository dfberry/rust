
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::chrono::DateTime;
use serde_with::chrono::Utc;


#[allow(non_snake_case)]
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub category: String,
    pub completedAt: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
}