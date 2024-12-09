use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

pub mod schema;

use crate::schema::myuser;

#[derive(Queryable, Debug, Serialize, Selectable, Deserialize)]
#[diesel(table_name = myuser)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = myuser)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub async fn create_user(connection: &mut PgConnection, name: &str, description: &str) -> User {
    use crate::schema::myuser;

    let new_user = NewUser {
        name: &name,
        description: &description,
    };

    diesel::insert_into(myuser::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user")

}
pub async fn get_user(
    connection: &mut PgConnection,
    user_id: &Uuid,
) -> Option<User> {
    use crate::schema::myuser::dsl::*;

    println!("get_user User: {:?}", user_id);

    let users = myuser
    .filter(id.eq(user_id))
        .limit(1)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

        // consume the Vec<User> and return the first item directly
        if let Some(user) = users.into_iter().next() {
            println!("Display user: {:?}", user);
            Some(user)
        } else {
            println!("No user found with github_user_id: {}", user_id);
            None
        }
}

pub async fn get_users(connection: &mut PgConnection) -> Vec<User> {
    use crate::schema::myuser::dsl::*;

    myuser
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users")
}
pub fn delete_user(connection: &mut PgConnection, user_id: &str) -> usize {
    use crate::schema::myuser::dsl::*;

    diesel::delete(myuser)
        .execute(connection)
        .expect("Error deleting user")
}
#[tokio::main]
async fn main() {

    use self::schema::myuser::dsl::*;

    let mut connection = establish_connection();

    let user = create_user(&mut connection, "Alice", "Alice's description").await;
    println!("Created user {:?}", user);

    let found_user = get_user(&mut connection, &user.id).await;
    println!("Found user {:?}", found_user);

    let user2 = create_user(&mut connection, "Bob", "Bob's description").await;
    println!("Created user {:?}", user2);

    let results = myuser
        .limit(10)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading posts");


    println!("Displaying {} users", results.len());

    for user in results {
        //delete_user(&mut connection, &user.id);
        println!("delete {}", user.name);
    }
}