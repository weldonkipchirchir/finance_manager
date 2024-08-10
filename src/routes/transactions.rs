use crate::model::{NewTransaction, Transaction};
use crate::repositories::TransactionsRepository;
use crate::{AuthenticatedUser, DBConnection};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, response::status::Custom};
use serde_json::{json, Value};

#[post("/transaction", format = "json", data = "<new_transaction>")]
pub async fn create_transaction(
    db: DBConnection,
    auth: AuthenticatedUser,
    new_transaction: Json<NewTransaction>,
) -> Result<Custom<Value>, Custom<Value>> {
    let mut transaction = new_transaction.into_inner();
    transaction.user_id = Some(auth.id);

    match transaction.validate() {
        Ok(()) => {
            db.run(
                move |c| match TransactionsRepository::create_transaction(c, transaction) {
                    Ok(transaction) => Ok(Custom(Status::Created, json!({"message": transaction}))),
                    Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
                },
            )
            .await
        }
        Err(error) => Err(Custom(Status::BadRequest, json!({"errors":error}))),
    }
}
