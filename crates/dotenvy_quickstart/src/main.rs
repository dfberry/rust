use dotenvy::dotenv;
use std::env;

fn main() {

    //if env::var("ENVIRONMENT").unwrap_or_else(|_| "".to_string()).to_lowercase() == "development" {
    dotenv().ok();
    let pat = env::var("API_KEY");
    match pat {
        Ok(val) => println!("API_KEY: {}", val),   
        Err(e) => println!("Couldn't read API_KEY: {}", e),
    }

}
