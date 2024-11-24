use std::env;
use tokio_postgres::NoTls;
use dotenvy::dotenv;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL")?;

    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Execute a simple query.
    let rows = client.query("SELECT * FROM user", &[]).await?;

    // Process the rows (for demonstration purposes, we'll just print them).
    for row in rows {
        let id: i32 = row.get("id");
        let username: &str = row.get("username");
        println!("id: {}, name: {}", id, username);
    }

    Ok(())
}