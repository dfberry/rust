use mongodb::Collection;
use crate::models::database_collection_item::ItemModel;
use crate::models::response::ItemResponse;
use crate::models::database_error::MyDBError;
use crate::models::transform::database_to_response;
use crate::db::fetch_notes;
use std::error::Error;

pub async fn get_all_items(collection: &Collection<ItemModel>) -> Result<Vec<ItemResponse>, Box<dyn Error>> {

    let limit = 10;
    let page = 1;

    match fetch_notes(&collection, limit, page).await {
        Ok(res) => {
            println!("fetch_notes was successful");
            let transformed_res = res.into_iter().map(database_to_response).collect::<Result<Vec<_>, _>>();
            
            match &transformed_res {
                Ok(_) => println!("Transformation was successful"),
                Err(_) => println!("Transformation failed"),
            }

            transformed_res
        },
        Err(e) => {
            println!("fetch_notes failed");
            Err(e.into())
        },
    }

} 

