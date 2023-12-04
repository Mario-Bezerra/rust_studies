pub mod rustaceans;
pub mod crates;
pub mod authorization;

use std::error::Error;
use rocket::{response::status::Custom, outcome::Outcome};
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

use rocket::request::{self, Request, FromRequest};
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use crate::models::{User, RoleCode};
use crate::repositories::{UserRepository, RoleRepository};
use rocket_db_pools::Connection;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        
        // Authorization : Bearer
        let name_of_parameter_for_authorization = "Authorization";
        let type_of_authorization = "Bearer";
        let session_header = req.headers().get_one(name_of_parameter_for_authorization)
            .map(|vector| vector.split_whitespace().collect::<Vec<_>>())
            .filter(|vector| vector.len() == 2 && vector[0] == type_of_authorization );

        if let Some(header_value) = session_header {
            let mut cache = req.guard::<Connection<CacheConn>>().await
                .expect("Can not connect to Redis in request Guard");
            let mut db = req.guard::<Connection<DbConn>>().await
                .expect("Can not connect to Postgres in request guard");

            let result = cache.get::<String, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = req.guard::<User>().await
            .expect("Cannot retrieve current logged in user");
        let mut db = req.guard::<Connection<DbConn>>().await
            .expect("Can not connect to Postgres in request guard");

        if let Ok(roles) = RoleRepository::find_by_user(&mut db, &user).await {
            rocket::info!("Roles assigned are {:?}", roles);
            let is_editor = roles.iter().any(|r| match r.code {
                RoleCode::Admin => true,
                RoleCode::Editor => true,
                _ => false
            });
            rocket::info!("Is editor is {:?}", is_editor);

            if is_editor {
                return Outcome::Success(EditorUser(user));
            }
        }


        Outcome::Error((Status::Unauthorized, ()))
    }
}
