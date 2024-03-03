// include the services module
//pub mod services; 
pub mod database;
pub mod version;

#[tokio::main]
async fn main() {
    let my_string = version::get_async_hello().await;
    println!("{}", my_string);

    let my_string_2 = database::get_async_hello_2().await;
    println!("{}", my_string_2);
}
