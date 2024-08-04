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
                finance_manager::routes::budget::view_budgets,
                finance_manager::routes::transactions::create_transaction,
                finance_manager::routes::transactions::view_transaction,
                finance_manager::routes::transactions::view_transactions,
                finance_manager::routes::transactions::update_transaction,
                finance_manager::routes::transactions::delete_transaction
            ],
        )
        .attach(finance_manager::DBConnection::fairing())
        .launch()
        .await;
}
