use crate::model::{NewTransaction, UpdateTransaction};
use crate::repositories::TransactionsRepository;
use crate::{AuthenticatedUser, DBConnection};
use rocket::http::Status;
use rocket::serde::json::{self, Json};
use rocket::{delete, put};
use rocket::{get, post, response::status::Custom};
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

#[get("/transactions")]
pub async fn view_transactions(
    db: DBConnection,
    _auth: AuthenticatedUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(
        move |c| match TransactionsRepository::find_multiple_transactions(c, 100) {
            Ok(Some(transactions)) => Ok(Custom(Status::Ok, json!(transactions))),
            Ok(None) => Err(Custom(
                Status::NotFound,
                json!({"error": "Transaction not found"}),
            )),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                json!({"error":"something went wrong"}),
            )),
        },
    )
    .await
}

#[get("/transaction/<id>")]
pub async fn view_transaction(
    db: DBConnection,
    _auth: AuthenticatedUser,
    id: i32,
) -> Result<Value, Custom<Value>> {
    db.run(
        move |c| match TransactionsRepository::find_transaction(c, id) {
            Ok(Some(transaction)) => Ok(json!(transaction)),
            Ok(None) => Err(Custom(
                Status::NotFound,
                json!({"error":"Transaction not found"}),
            )),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                json!({"error":"something went wrong"}),
            )),
        },
    )
    .await
}

#[put("/transaction/<id>", format = "json", data = "<transaction>")]
pub async fn update_transaction(
    db: DBConnection,
    _auth: AuthenticatedUser,
    id: i32,
    transaction: Json<UpdateTransaction>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(
        move |c| match TransactionsRepository::find_transaction(c, id) {
            Ok(Some(_)) => match transaction.validate() {
                Ok(()) => match TransactionsRepository::update_transaction(
                    c,
                    id,
                    transaction.into_inner(),
                ) {
                    Ok(transaction_res) => {
                        Ok(Custom(Status::Ok, json!({"message":transaction_res})))
                    }
                    Err(_) => Err(Custom(
                        Status::InternalServerError,
                        json!({"error":"something went wrong"}),
                    )),
                },
                Err(errors) => Err(Custom(Status::BadRequest, json!({"error":errors}))),
            },
            Ok(None) => Err(Custom(
                Status::NotFound,
                json!({"error":"Transaction not found"}),
            )),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                json!({"errror":"something went wrong"}),
            )),
        },
    )
    .await
}

#[delete("/transaction/<id>")]
pub async fn delete_transaction(
    db: DBConnection,
    _auth: AuthenticatedUser,
    id: i32,
) -> Result<Value, Custom<Value>> {
    db.run(
        move |c| match TransactionsRepository::find_transaction(c, id) {
            Ok(Some(_)) => match TransactionsRepository::delete_transaction(c, id) {
                Ok(_) => Ok(json!({"message":"transaction deleted"})),
                Err(err) => {
                    eprintln!("Error fetching budget: {:?}", err);
                    Err(Custom(
                        Status::InternalServerError,
                        json!("Something went wrong"),
                    ))
                }
            },
            Ok(None) => Err(Custom(
                Status::NotFound,
                json!({"error": "transaction not found"}),
            )),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                json!({"error":"Something went wrong"}),
            )),
        },
    )
    .await
}
