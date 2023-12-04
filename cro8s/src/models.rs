use std::io::Write;
use std::str::FromStr;
use chrono::NaiveDateTime;
use diesel::deserialize::{FromSqlRow, FromSql, self};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{ToSql, Output, self};
use diesel::sql_types::Text;
use diesel::{Insertable, Queryable};
use diesel::query_builder::AsChangeset; 
use diesel::associations::{Associations, Identifiable}; 
use diesel::expression::AsExpression;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name:String,
    pub email:String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Queryable, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Identifiable, Debug)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(AsExpression, Debug, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => String::from("admin"),
            RoleCode::Editor => String::from("editor"),
            RoleCode::Viewer => String::from("viewer"),
        }
    }
}

impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        match value.as_bytes(){
            b"admin" => Ok(RoleCode::Admin),
            b"editor" => Ok(RoleCode::Editor),
            b"viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer),
        }
    }
}

impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err(()),
        }
    }
}

impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
        };
        Ok(diesel::serialize::IsNull::No)
    }
}