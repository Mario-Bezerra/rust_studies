pub mod rustaceans;
pub mod crates;
pub mod authorization;

use std::error::Error;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::json::json;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(error: Box<dyn Error>) -> Custom<rocket::serde::json::Value>{
    rocket::error!("{}", error);
    Custom(Status::InternalServerError, json!("Error"))
}