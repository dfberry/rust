use postgres::Client;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::error;
fn main() -> Result<(), Box<dyn error::Error>> {
    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());
    let mut client = Client::connect("postgresql://neondb_owner:YQESolK7vbW6@ep-cold-water-a5pddjz8.us-east-2.aws.neon.tech/neondb?sslmode=require&options=endpoint%3Dep-cold-water-a5pddjz8", connector)?;
    for row in client.query("SELECT 42", &[])? {
        let ret : i32 = row.get(0);
        println!("Result = {}", ret);
    }
    Ok(())
}