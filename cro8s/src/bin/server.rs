extern crate cro8s;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
            .mount("/", rocket::routes![
                cro8s::routes::authorization::login,
                cro8s::routes::rustaceans::get_rustaceans,
                cro8s::routes::rustaceans::get_one_rustacean,
                cro8s::routes::rustaceans::create_rustacean,
                cro8s::routes::rustaceans::update_rustacean,
                cro8s::routes::rustaceans::delete_rustacean,
                cro8s::routes::crates::get_crates,
                cro8s::routes::crates::get_one_crate,
                cro8s::routes::crates::create_crate,
                cro8s::routes::crates::update_crate,
                cro8s::routes::crates::delete_crate,
            ])
            .attach(crate::cro8s::routes::DbConn::fairing())
            .launch()
            .await;
}