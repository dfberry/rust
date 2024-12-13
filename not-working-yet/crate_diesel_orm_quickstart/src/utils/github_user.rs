use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::Utc;
use dotenvy::dotenv;
use std::env;

use super::models::GitHub_User;
use super::schema::github_users;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_user(conn: &mut PgConnection, id: &str, github_id: &str, username: &str) -> GitHub_User {

    let new_user = GitHub_User {
        id: id.to_string(),
        github_id: github_id.to_string(),
        username: username.to_string(),
        created_at: Utc::now(),
    };

    diesel::insert_into(github_users::table)
        .values(&new_user)
        .returning(GitHub_User::as_returning())
        .get_result(conn)
        .expect("Error saving new GitHub_User")

}
pub fn read_all_users(conn: &mut PgConnection,) -> QueryResult<Vec<GitHub_User>> {

    github_users::table.load::<GitHub_User>(conn)

/*
    use crate_diesel_orm_quickstart::schema::github_users::dsl::*;

    let connection = &mut establish_connection();
    let results = github_users
        .select(GitHub_User::as_select())
        .load(connection)
        .expect("Error loading github users");

    println!("Displaying {} github users", results.len());
    for github_user in results {
        println!("{}", github_user.id);
        println!("-----------\n");
        println!("{}", github_user.username);
        println!("{}", github_user.github_id);
    }
     */


}
/*
pub fn read_user(conn: &PgConnection, user_id: &str) -> QueryResult<GitHub_User> {

    github_users.filter(id.eq(user_id))
        .first(conn)
}



pub fn update_user(conn: &PgConnection, user_id: &str, new_username: &str) -> QueryResult<usize> {

    diesel::update(github_users.filter(id.eq(user_id)))
        .set(username.eq(new_username))
        .execute(conn)
}

pub fn delete_user(conn: &PgConnection, user_id: &str) -> QueryResult<usize> {

    diesel::delete(github_users.filter(id.eq(user_id)))
        .execute(conn)
}



use diesel_demo::establish_connection;
use diesel_demo::github_user_crud::*;

fn main() {
    let connection = &mut establish_connection();

    // Create a new user
    let new_user = create_user(connection, "1", "github_1", "user1")
        .expect("Error creating new user");

    println!("Created user: {:?}", new_user);

    // Read a user
    let user = read_user(connection, "1")
        .expect("Error reading user");

    println!("Read user: {:?}", user);

    // Update a user
    let updated_rows = update_user(connection, "1", "new_username")
        .expect("Error updating user");

    println!("Updated {} rows", updated_rows);

    // List all users
    let users = read_all_users(connection)
        .expect("Error reading all users");

    println!("All users: {:?}", users);

    // Delete a user
    let deleted_rows = delete_user(connection, "1")
        .expect("Error deleting user");

    println!("Deleted {} rows", deleted_rows);
}

        */