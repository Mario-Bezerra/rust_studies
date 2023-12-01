use rocket::{serde::json::{Json, json}, response::status::Custom, http::Status};
use rocket_db_pools::{Connection, deadpool_redis::redis::AsyncCommands};
use serde_json::Value;
use crate::{repositories::UserRepository, auth::{authorize_user, Credentials}, routes::CacheConn};
use super::DbConn;
use super::server_error;

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, mut cache: Connection<CacheConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_username(&mut db, &credentials.username).await
        .map_err(|e| server_error(e.into()))?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache.set_ex::<String, i32, ()>(
        format!("sessions/{}", session_id),
        user.id,
        3*60*60)
    .await
    .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id,
    }))
} 