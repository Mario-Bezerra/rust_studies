use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::password_hash::PasswordHasher;
use diesel::{PgConnection, Connection};

use crate::{models::NewUser, repositories::{UserRepository, RoleRepository}};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot retrieve Database URL from environment");
    diesel::PgConnection::establish(&database_url)
        .expect("Cannot connect to postgres")
}

pub fn create_user(username : String, password: String, roles: Vec<String>){
    let mut conn = load_db_connection();
    let salt = SaltString::generate(OsRng);
    let argon2 = argon2::Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    let new_user = NewUser { username, password: password_hash.to_string() };
    let user = UserRepository::create(&mut conn, new_user, roles).unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut conn, &user).unwrap();
    println!("Roles assigned {:?}", roles);
}

pub fn list_users(){
    let mut conn = load_db_connection();

    let users = UserRepository::find_with_roles(&mut conn).unwrap();
    for user in users {
        println!("{:?}", user);
    }

}

pub fn delete_user(id: i32){
    let mut conn = load_db_connection();
    UserRepository::delete(&mut conn, id).unwrap();
}