use diesel_async::{AsyncPgConnection, AsyncConnection};
use crate::auth;
use crate::{models::NewUser, repositories::{UserRepository, RoleRepository}};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username : String, password: String, roles: Vec<String>){
    let mut conn = load_db_connection().await;
    
    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser { username, password: password_hash };

    let user = UserRepository::create(&mut conn, new_user, roles).await.unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut conn, &user).await.unwrap();
    println!("Roles assigned {:?}", roles);
}

pub async fn list_users(){
    let mut conn = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut conn).await.unwrap();
    for user in users {
        println!("{:?}", user);
    }

}

pub async fn delete_user(id: i32){
    let mut conn = load_db_connection().await;
    UserRepository::delete(&mut conn, id).await.unwrap();
}