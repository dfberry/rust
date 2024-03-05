use mongodb::{Client, Collection, bson::{doc, Document}, error::Result};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use futures::stream::TryStreamExt;

// ----------------------------------------
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct UpdateError {
    details: String
}

impl UpdateError {
    fn new(msg: &str) -> UpdateError {
        UpdateError{details: msg.to_string()}
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for UpdateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
// ----------------------------------------

pub struct Crud2<T> {
    collection: Arc<Mutex<Collection<T>>>,
}

impl<T: Serialize + Deserialize<'static> + Unpin + Send + Sync> Crud2<T> {
    pub async fn new(connection_string: &str, database_name: &str, collection_name: &str) -> Result<Self> {
        let client = Client::with_uri_str(connection_string).await?;
        let database = client.database(database_name);
        let collection = database.collection::<T>(collection_name);

        Ok(Self { collection: Arc::new(Mutex::new(collection)) })
    }

    pub async fn create(&self, document: T) -> Result<String> {
        let collection = self.collection.lock().unwrap();
        let insert_result = collection.insert_one(document, None).await?;

        Ok(insert_result.inserted_id.as_object_id().unwrap().to_hex())

    }

    pub async fn read(&self, filter: Document) -> Result<Vec<T>> {
        let collection = self.collection.lock().unwrap();
        let cursor = collection.find(filter, None).await?;
        let mut docs = Vec::new();

        while let Some(result) = cursor.try_next().await? {
            docs.push(result)
        }
    
        Ok(docs)
    }
    pub async fn read_by_id(&self, id: String) -> Result<Option<T>> {
        let collection = self.collection.lock().unwrap();
        let filter = doc! { "_id": mongodb::bson::oid::ObjectId::from_str(&id).unwrap() };
        let result = collection.find_one(filter, None).await?;
        Ok(result)
    }

    pub async fn update(&self, filter: Document, update: Document) -> Result<()> {
        let collection = self.collection.lock().unwrap();
        let update_result = collection.update_one(filter, update, None).await?;
        if update_result.matched_count != 1 {
            return Err();
        }
        Ok(())
    }
    pub async fn update_by_id(&self, id: String, update: Document) -> Result<()> {
        let collection = self.collection.lock().unwrap();
        let filter = doc! { "_id": mongodb::bson::oid::ObjectId::from_str(&id).unwrap() };
        let update_result = collection.update_one(filter, update, None).await?;
        if update_result.matched_count != 1 {
            return Err(mongodb::error::Error::from(mongodb::error::ErrorKind::OperationError("No document was updated".to_string())));
        }
        Ok(())
    }

    pub async fn delete(&self, filter: Document) -> Result<()> {
        let collection = self.collection.lock().unwrap();
        let delete_result = collection.delete_one(filter, None).await?;
        if delete_result.deleted_count != 1 {
            return Err(mongodb::error::Error::from(mongodb::error::ErrorKind::OperationError("No document was deleted".to_string())));
        }
        Ok(())
    }
    pub async fn delete_by_id(&self, id: String) -> Result<()> {
        let collection = self.collection.lock().unwrap();
        let filter = doc! { "_id": mongodb::bson::oid::ObjectId::from_str(&id).unwrap() };
        let delete_result = collection.delete_one(filter, None).await?;
        if delete_result.deleted_count != 1 {
            return Err(mongodb::error::Error::from(mongodb::error::ErrorKind::OperationError("No document was deleted".to_string())));
        }
        Ok(())
    }
}