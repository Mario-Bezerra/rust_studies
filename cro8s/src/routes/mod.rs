pub mod rustaceans;
pub mod crates;

use std::error::Error;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::json::json;

use diesel::PgConnection;
#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub fn server_error(error: Box<dyn Error>) -> Custom<rocket::serde::json::Value>{
    rocket::error!("{}", error);
    Custom(Status::InternalServerError, json!("Error"))
}