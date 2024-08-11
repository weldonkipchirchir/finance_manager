extern crate finance_manager;
use std::str::FromStr;

use rocket::{self, routes};
use rocket_cors::{CorsOptions, AllowedHeaders, AllowedOrigins, AllowedMethods};

#[rocket::main]
async fn main() {
    let allowed_methods : AllowedMethods = ["Get", "Post", "Put", "Delete"]
    .iter()
    .map(|s| FromStr::from_str(s).unwrap())
    .collect();

    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["http://localhost:3000"]),
        allowed_methods: allowed_methods,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS");

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                finance_manager::routes::user::create_user,
                finance_manager::routes::user::login,
                finance_manager::routes::budget::create_budget,
                finance_manager::routes::budget::update_budget,
                finance_manager::routes::budget::view_budget,
                finance_manager::routes::budget::delete_budget,
                finance_manager::routes::budget::view_budgets,
                finance_manager::routes::transactions::create_transaction,
                finance_manager::routes::transactions::view_transactions,
                // finance_manager::routes::transactions::view_transaction,
                // finance_manager::routes::transactions::update_transaction,
                // finance_manager::routes::transactions::delete_transaction
            ],
        )
        .attach(finance_manager::DBConnection::fairing())
        .attach(cors)
        .launch()
        .await;
}
