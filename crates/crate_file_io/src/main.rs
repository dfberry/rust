use serde_json::json;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use tokio;

pub async fn write_json_to_file(
    file_name_and_path: &str, 
    json_blob: &Value
) -> Result<(), std::io::Error> {

    println!("Writing JSON to file: {}", file_name_and_path);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .open(file_name_and_path)?;

    match file.write_all(serde_json::to_string_pretty(json_blob)?.as_bytes()) {
        Ok(_) => println!("Successfully wrote JSON to file: {}", file_name_and_path),
        Err(e) => println!("Failed to write JSON to file: {}. Error: {:?}", file_name_and_path, e),
    }

    Ok(())
}
#[tokio::main]
async fn main() {

    let my_json: Value = json!({
        "a": 123,
        "b": "hello"
    });

    if let Err(e) = write_json_to_file("./data/data_1.json", &my_json).await {
        eprintln!("Error writing JSON to file: {:?}", e);
    }
}

