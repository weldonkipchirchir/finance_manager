extern crate finance_manager;
use rocket::{self, routes};

#[rocket::main]
async fn main() {
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
                finance_manager::routes::budget::view_budgets
            ],
        )
        .attach(finance_manager::DBConnection::fairing())
        .launch()
        .await;
}
