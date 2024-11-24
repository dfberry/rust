use diesel::prelude::*;
use rand::Rng;

mod utils;

use utils::github_user::establish_connection;
use utils::models::*;
use utils::schema::github_users::dsl::*;

fn generate_random_string() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789";
    const STRING_LEN: usize = 8;
    let mut rng = rand::thread_rng();

    let random_string: String = (0..STRING_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    random_string
}


fn main() {

    let connection = &mut establish_connection();


    // List all users
    let results = github_users
        .select(GitHub_User::as_select())
        .load(connection)
        .expect("Error loading github users");


    for github_user in results {
        println!("-----------\n");
        println!("{}", github_user.id);
        println!("{}", github_user.username);
        println!("{}", github_user.github_id);
    }

    // Create a new user, where the id is a guid
    let new_user = GitHub_User {
        id: uuid::Uuid::new_v4().to_string(),
        github_id: generate_random_string(),
        username: generate_random_string(),
        created_at: chrono::Utc::now(),
    };

    diesel::insert_into(github_users)
        .values(&new_user)
        .returning(GitHub_User::as_returning())
        .get_result(connection)
        .expect("Error saving new GitHub_User");

    // List all users
    let results = github_users
        .select(GitHub_User::as_select())
        .load(connection)
        .expect("Error loading github users");

    for github_user in results {
        println!("-----------\n");
        println!("{}", github_user.id);
        println!("{}", github_user.username);
        println!("{}", github_user.github_id);
    }

    // Delete 1 user
    let user_to_delete = github_users
        .filter(id.eq(&new_user.id))
        .first::<GitHub_User>(connection)
        .expect("Error loading github user");

     // List all users
     let results = github_users
     .select(GitHub_User::as_select())
     .load(connection)
     .expect("Error loading github users");

    for github_user in results {
        println!("-----------\n");
        println!("{}", github_user.id);
        println!("{}", github_user.username);
        println!("{}", github_user.github_id);
 }   

}