#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct ListStruct {
    name: String,
    created_date: String,
    updated_date: String,
    _id: String, // underscore in cosmos, not underscore returned to client
}

#[derive(Debug)]
struct Err {}
impl From<mongodb::error::Error> for Err {
    fn from(_error: mongodb::error::Error) -> Self {
        Err {}
    }
}

#[allow(dead_code)]
type Result<T> = std::result::Result<T, Err>;

#[tokio::main]
async fn main() -> Result<()> {
        let client_options = ClientOptions::parse(
            "mongodb://",
        )
        .await?;
        let client = Client::with_options(client_options)?;
        let database = client.database("Todo");

        for collection_name in database.list_collection_names(None).await? {
            println!("{}", collection_name);
            let collection = database.collection::<Document>(&collection_name);

        }

        let nontyped_collection = database.collection::<Document>("TodoList");

        use futures::stream::TryStreamExt;
        use mongodb::{bson::doc, options::FindOptions};

        let mut cursor = nontyped_collection.find(None, None).await.expect("error occured"); 
        while let Some(doc) = cursor.try_next().await.unwrap() { 
            println!("{:?}", doc) 
        } 
    
        Ok(())
}
