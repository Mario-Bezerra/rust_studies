use rocket::response::status::{Custom, NoContent}; 
use rocket::http::Status; 
use rocket::serde::json::{json, Value, Json};

use super::{DbConn, server_error};
use crate::repositories::CrateRepository; 
use crate::models::{NewCrate, Crate};

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn) -> Result<Value, Custom<Value>>{
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::get("/crates/<id>")]
pub async fn get_one_crate(db: DbConn, id: i32) -> Result<Value, Custom<Value>>{
    db.run( move |c| {
        CrateRepository::find(c, id)
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::post("/crates", format="json", data="<new_crate>")]
pub async fn create_crate(db: DbConn, new_crate: Json<NewCrate>) -> Result<Custom<Value>, Custom<Value>>{
    db.run( move |c| {
        CrateRepository::create(c, new_crate.into_inner())
        .map(|crates| Custom(Status::Created, json!(crates)))
        .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::put("/crates/<id>", format="json", data="<a_crate>")]
pub async fn update_crate(db: DbConn, id: i32, a_crate: Json<Crate>) -> Result<Value, Custom<Value>>{
    db.run( move |c| {
        CrateRepository::update(c, id, a_crate.into_inner())
        .map(|crates| json!(crates))
        .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>>{
    db.run( move |c| {
        CrateRepository::delete(c, id)
        .map(|_| NoContent)
        .map_err(|e| server_error(e.into()))
    }).await
}