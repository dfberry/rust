// include the services module
pub mod services; 

#[tokio::main]
async fn main() {
    let my_string = services::get_async_hello().await;
    println!("{}", my_string);
}
