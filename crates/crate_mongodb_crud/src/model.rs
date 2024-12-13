use dotenv::dotenv;
use serde::Serialize;
use serde::Deserialize;
use mongodb::Collection;
use mongodb::Client;
use mongodb::bson::doc;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub category: String
}