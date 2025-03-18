//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```
use axum::{
    Json,
    response::{IntoResponse, Response},
    http::{self, StatusCode},
    body::Body,
    routing::{get, post}, 
    Router
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Queryable;
use diesel::Insertable;

use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

pub mod schema;
use crate::schema::test_table_optional_datetime;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::test_table_optional_datetime)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub optional_date_created: Option<NaiveDateTime>,
    pub date_created: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = crate::schema::test_table_optional_datetime)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewItem {
    pub name: String,
    pub optional_date_created: Option<NaiveDateTime>
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("PG_DATABASE_URL").expect("PG_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new()
    .route("/", get(get_all_items_handler))
    .route("/insert", post(insert_item_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
pub async fn get_items(connection: &mut PgConnection) -> Result<Vec<Item>, diesel::result::Error> {
    test_table_optional_datetime::table
        .limit(5)
        .select(Item::as_select())
        .load(connection)
}
pub async fn get_all_items_handler() -> impl IntoResponse {
    println!("handler");

    let mut connection = establish_connection();
    println!("connection");

    match get_items(&mut connection).await {
        Ok(items) => {
            let json = json!({ "items": items });
            println!("json: {}", json);
            Response::builder()
                .status(StatusCode::OK)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(json.to_string()))
                .unwrap()
        },
        Err(e) => {
            println!("Error loading items: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    }
}

pub async fn insert_item(connection: &mut PgConnection, new_item: &NewItem) -> Result<Item, diesel::result::Error> {
    diesel::insert_into(test_table_optional_datetime::table)
        .values(new_item)
        .get_result(connection)
}
pub async fn insert_item_handler(Json(item): Json<NewItem>) -> impl IntoResponse {
    let mut connection = establish_connection();
    println!("connection");

    match insert_item(&mut connection, &item).await {
        Ok(item) => {
            println!("Inserted item: {:?}", item);
            Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::empty())
                .unwrap()
        },
        Err(e) => {
            println!("Error inserting item: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()
        }
    }
}

