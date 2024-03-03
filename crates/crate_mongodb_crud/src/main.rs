use dotenv::dotenv;
use serde::Serialize;
use serde::Deserialize;
use mongodb::Collection;
use mongodb::Client;
use mongodb::bson::doc;
use rand::Rng;

pub mod crud;

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    category: String
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let connection_string =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.");
    let database_name = "Test";
    let collection_name = "Items";
    
    let client = Client::with_uri_str(connection_string).await.unwrap();
    let database = client.database(database_name);
    let collection: Collection<Item> = database.collection(collection_name);

    // create randmom number
    let mut rng = rand::thread_rng();
    let random_number: String = rng.gen_range(0..100).to_string().parse().unwrap();

    tests_by_name(&random_number, &collection).await;
    tests_by_id(&random_number, &collection).await;

}

async fn tests_by_name(rand_string: &String, collection: &Collection<Item>){

    let name = format!("item by name {}", rand_string);
    let category:String = "dog".to_string();

    // Add - for name
    let new_item = Item {
        name: name.clone(),
        category: category.clone()
    };
    let create_result = crate::crud::create(&collection, &new_item).await;
    let id = create_result.unwrap();
    println!("Inserted id: {}", id);

    // Read by name
    let filter = doc! { "name": name.clone(), "category": category.clone()};
    let items = crate::crud::read(&collection, filter).await;
    println!("Items: {:?}", items.unwrap());

     // update by name
    let updated_name = format!("item by name updated {}", rand_string);
    let filter = doc! { "name": name.clone(), "category": category.clone()};
    let update = doc! { "$set": { "name": &updated_name} };
    let result = crate::crud::update(&collection, filter.clone(), update).await;
    println!("Update result: {:?}", result.unwrap());

     // delete by name
     let delete_filter = doc! { "name": &updated_name, "category": category.clone()};
     let result = crate::crud::delete(&collection, delete_filter).await;
     println!("Delete result: {:?}", result.unwrap());
   
}
async fn tests_by_id(rand_string: &String, collection: &Collection<Item>){

    let name = format!("item by id {}", rand_string);
    let category:String = "cat".to_string();

    let new_item = Item {
        name: name.clone(),
        category:category.clone()
    };
    let create_result = crate::crud::create(&collection, &new_item).await;
    let id = create_result.unwrap();
    println!("Inserted id: {}", id);

    // Read by id
    let items = crate::crud::read_by_id(&collection, &id).await;
    println!("Items: {:?}", items.unwrap());

    // Update by id
    let updated_name = format!("item by id updated {}", rand_string);
    let update_document = doc! { "$set": { "name": &updated_name } };
    let update_result = crate::crud::update_by_id(&collection, &id, update_document.clone()).await;
    println!("Update result: {:?}", update_result.unwrap());

    // delete by id
    let delete_result = crate::crud::delete_by_id(&collection, &id).await;
    println!("Delete result: {:?}", delete_result.unwrap());
}