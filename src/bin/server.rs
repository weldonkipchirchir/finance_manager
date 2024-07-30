extern crate finance_manager;
use rocket::{self, routes};

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                finance_manager::routes::user::create_user,
                finance_manager::routes::user::login
            ],
        )
        .attach(finance_manager::DBConnection::fairing())
        .launch()
        .await;
}
