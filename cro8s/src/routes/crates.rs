use rocket::response::status::{Custom, NoContent}; 
use rocket::http::Status; 
use rocket::serde::json::{json, Value, Json};
use rocket_db_pools::Connection;

use super::{DbConn, server_error};
use crate::repositories::CrateRepository; 
use crate::models::{NewCrate, Crate};

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>>{
        CrateRepository::find_multiple(&mut db, 100).await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn get_one_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>>{
        CrateRepository::find(&mut db, id).await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCrate>) -> Result<Custom<Value>, Custom<Value>>{
        CrateRepository::create(&mut db, new_crate.into_inner()).await
        .map(|crates| Custom(Status::Created, json!(crates)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, a_crate: Json<Crate>) -> Result<Value, Custom<Value>>{
        CrateRepository::update(&mut db, id, a_crate.into_inner()).await
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>>{
        CrateRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
}