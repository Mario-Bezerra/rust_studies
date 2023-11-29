use argon2::{PasswordHash, PasswordVerifier};
use rocket::{serde::json::{Json, json}, response::status::Custom};
use serde::Deserialize;
use serde_json::Value;
use crate::repositories::UserRepository;
use super::DbConn;
use super::server_error;

#[derive(Deserialize)]
pub struct Credentials {
    pub username : String,
    pub password : String,
}

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(db : DbConn, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {UserRepository::find_by_username(c, &credentials.username)
        .map(|user| {
            let argon2 = argon2::Argon2::default();
                let db_hash = PasswordHash::new(&user.password).unwrap();
                let result = argon2.verify_password(credentials.password.as_bytes(), &db_hash);
                if result.is_ok() {
                    return json!("Success");
        }
                json!("Unauthorized")
        })
        .map_err(|e| server_error(e.into()))})
        .await
} 