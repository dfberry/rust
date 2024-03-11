
use dotenv::dotenv;
use mongodb::Collection;
use mongodb::Client;
use mongodb::bson::doc;
use rand::Rng;
use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use mongodb::bson::DateTime;

use bson::serde_helpers::serialize_object_id_as_hex_string;
use bson::serde_helpers::deserialize_hex_string_from_object_id;
use bson::Bson;


#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemModel {
    pub _id: ObjectId,
    pub name: String,
    pub category: String,
    pub completedAt: Option<DateTime>,
    pub createdAt: DateTime,
    pub updatedAt: DateTime,
}
impl ItemModel {
    pub fn new(name: String, category: String) -> Self {
      let now = bson::DateTime::now();
      Self {
        _id: ObjectId::new(),
        name: name,
        category: category,
        completedAt: Some(now),
        updatedAt: now,
        createdAt: now,
      }
    }
    pub fn read(&self) -> Bson {
      
      // convert _id from ObjectId to string
      let id = self._id.to_hex();

        // convert DateTime<Utc> to string
        let completedAt = match self.completedAt.as_ref() {
            Some(dt) => match dt.try_to_rfc3339_string() {
                Ok(s) => Some(s),
                Err(_) => None, // handle error as you see fit
            },
            None => None,
        };
        let createdAt = self.createdAt.try_to_rfc3339_string().unwrap();
        let updatedAt = self.updatedAt.try_to_rfc3339_string().unwrap();

        let doc = doc! {
            "_id": id,
            "name": self.name.clone(),
            "category": self.category.clone(),
            "completedAt": completedAt,
            "createdAt": createdAt,
            "updatedAt": updatedAt,
        };
        Bson::Document(doc)

    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv().ok();
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = "Test";
    let collection_name = "Items";
    
    let client = Client::with_uri_str(connection_string).await.unwrap();
    let database = client.database(database_name);
    let collection_item_model = database.collection::<ItemModel>(collection_name);

    // create random number
    let randNumber = rand::thread_rng().gen_range(0..100);
    let name = "Item ".to_string() + &randNumber.to_string();
    let category = "Category ".to_string() + &randNumber.to_string();

    let item = ItemModel::new(name, category);

    // insert item in database
    let result = collection_item_model.insert_one(item, None).await.unwrap();

    // get id from result
    let id = result.inserted_id.as_object_id().unwrap();

    println!("Inserted item with id: {}", id);

    // read item from database
    let item = collection_item_model.find_one(doc! { "_id": id }, None).await.unwrap().unwrap();
    // convert item to json
    //let json = bson::to_bson(&item).unwrap();
    let item_as_bson = item.read();
    // print json
    println!("{}", item_as_bson);

}
