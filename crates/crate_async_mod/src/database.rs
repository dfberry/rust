use tokio::time::{sleep, Duration};

pub async fn get_async_hello_2() -> String {
    // simulate some async operation
    sleep(Duration::from_secs(2)).await;
    "Hello from Rust 2!".to_string()
}