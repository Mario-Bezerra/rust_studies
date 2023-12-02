use rocket::{serde::json::{Json, json}, response::status::Custom};
use serde_json::Value;
use crate::{repositories::UserRepository, auth::{authorize_user, Credentials}};
use super::DbConn;
use super::server_error;

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(db : DbConn, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {UserRepository::find_by_username(c, &credentials.username)
        .map(|user| {
            if let Ok(token) = authorize_user(&user, credentials.into_inner()) {
                return json!(token);
        }
                json!("Unauthorized")
        })
        .map_err(|e| server_error(e.into()))})
        .await
} 