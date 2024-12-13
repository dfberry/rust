use serde::{Serialize, Deserialize};
use mongodb::{
    bson::{
        DateTime,
        serde_helpers::{
            serialize_bson_datetime_as_rfc3339_string, 
            serialize_hex_string_as_object_id,
            deserialize_bson_datetime_from_rfc3339_string
    }},
};
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub category: String,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string", deserialize_with = "deserialize_bson_datetime_from_rfc3339_string")]
    pub created_at: bson::DateTime
}