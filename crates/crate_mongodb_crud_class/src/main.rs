use dotenv::dotenv;
use serde::Serialize;
use serde::Deserialize;
use mongodb::bson::doc;
use rand::Rng;
use crate::crud2::Crud2;

pub mod crud2;


#[derive(Clone, Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    category: String
}

#[tokio::main]
async fn main() {

    dotenv().ok();
    let connection_string: String =
    std::env::var("MONGODB_CONNECTION_STRING").expect("MONGODB_CONNECTION_STRING must be set.").to_string();
    let database_name: String = "Test".to_string();
    let collection_name: String = "Items".to_string();
    
    let mut my_collection: Crud2<Item> = Crud2::new(&connection_string, &database_name, &collection_name);

    // create randmom number
    let mut rng = rand::thread_rng();
    let random_number: String = rng.gen_range(0..100).to_string().parse().unwrap();

    tests_by_name(&random_number, &mut my_collection).await;
    tests_by_id(&random_number, &mut my_collection).await;
}

async fn tests_by_name(rand_string: &String, my_collection: &mut MyCollection<Item>){

    let name = format!("item by name {}", rand_string);
    let category:String = "dog".to_string();

    // Add - for name
    let new_item = Item {
        name: name.clone(),
        category: category.clone()
    };


    let create_result = my_collection.create(&new_item).await;
    let id = create_result.unwrap();
    println!("Inserted id: {}", id);

    // // Read by name
    let filter = doc! { "name": name.clone(), "category": category.clone()};
    let items = my_collection.read(filter).await;
    println!("Items: {:?}", items.unwrap());

    //  // update by name
    let updated_name = format!("item by name updated {}", rand_string);
    let filter = doc! { "name": name.clone(), "category": category.clone()};
    let update = doc! { "$set": { "name": &updated_name} };
    let result = my_collection.update(filter.clone(), update).await;
    println!("Update result: {:?}", result.unwrap());

    //  // delete by name
     let delete_filter = doc! { "name": &updated_name, "category": category.clone()};
     let result = my_collection.delete(delete_filter).await;
     println!("Delete result: {:?}", result.unwrap());
   
}
async fn tests_by_id(rand_string: &String, collection: &mut MyCollection<Item>){

    let name = format!("item by id {}", rand_string);
    let category:String = "cat".to_string();

    let new_item = Item {
        name: name.clone(),
        category:category.clone()
    };
    let create_result = collection.create(&new_item).await;
    let id = create_result.unwrap();
    println!("Inserted id: {}", id);

    // // Read by id
    let items = collection.read_by_id(&id).await;
    println!("Items: {:?}", items.unwrap());

    // // Update by id
    let updated_name = format!("item by id updated {}", rand_string);
    let update_document = doc! { "$set": { "name": &updated_name } };
    let update_result = collection.update_by_id(&id, update_document.clone()).await;
    println!("Update result: {:?}", update_result.unwrap());

    // // delete by id
    let delete_result = collection.delete_by_id(&id).await;
    println!("Delete result: {:?}", delete_result.unwrap());
}