use diesel::prelude::*;
use crate_diesel_orm_quickstart::establish_connection;
use crate_diesel_orm_quickstart::models::*;
use crate_diesel_orm_quickstart::schema::github_users::dsl::*;


fn main() {
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
}