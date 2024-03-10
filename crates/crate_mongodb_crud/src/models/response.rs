use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ItemResponse {
    pub id: String,
    pub name: String,
    pub category: String,
    pub completedAt: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
}

#[derive(Serialize, Debug)]
pub struct ItemData {
    pub item: ItemResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleItemResponse {
    pub status: &'static str,
    pub data: ItemData,
}

#[derive(Serialize, Debug)]
pub struct ItemListResponse {
    pub status: &'static str,
    pub results: usize,
    pub items: Vec<ItemResponse>,
}
