mod models;
mod schema;
mod repositories;
mod routes;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
            .mount("/", rocket::routes![
                routes::rustaceans::get_rustaceans,
                routes::rustaceans::get_one_rustacean,
                routes::rustaceans::create_rustacean,
                routes::rustaceans::update_rustacean,
                routes::rustaceans::delete_rustacean,
                routes::crates::get_crates,
                routes::crates::get_one_crate,
                routes::crates::create_crate,
                routes::crates::update_crate,
                routes::crates::delete_crate,
            ])
            .attach(crate::routes::DbConn::fairing())
            .launch()
            .await;
}
