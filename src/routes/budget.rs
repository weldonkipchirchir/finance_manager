use crate::model::{NewBudget, UpdateBudget};
use crate::repositories::BudgetRepository;
use crate::{AuthenticatedUser, DBConnection};
use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::status::Custom,
    serde::json::{serde_json::json, Json},
};
use serde_json::Value;

#[post("/budget", format = "json", data = "<new_budget>")]
pub async fn create_budget(
    db: DBConnection,
    auth: AuthenticatedUser,
    new_budget: Json<NewBudget>,
) -> Result<Custom<Value>, Custom<Value>> {
    let mut budget = new_budget.into_inner();
    budget.user_id = Some(auth.id);

    match budget.validate() {
        Ok(()) => {
            db.run(move |c| match BudgetRepository::create_budget(c, budget) {
                Ok(budget_res) => Ok(Custom(Status::Created, json!({"message":budget_res}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
            })
            .await
        }
        Err(error) => {
            // Validation failed, return an error response
            Err(Custom(Status::BadRequest, json!({"errors": error})))
        }
    }
}

#[get("/budget/<id>")]
pub async fn view_budget(
    db: DBConnection,
    _auth: AuthenticatedUser,
    id: i32,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| match BudgetRepository::find_budget(c, id) {
        Ok(Some(budget)) => Ok(json!(budget)),
        Ok(None) => Err(Custom(
            Status::NotFound,
            json!({"error": "Budget not found"}),
        )),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            json!({"error":"Something went wrong"}),
        )),
    })
    .await
}

#[get("/budget")]
pub async fn view_budgets(
    db: DBConnection,
    _auth: AuthenticatedUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(
        move |c| match BudgetRepository::find_multiple_budgets(c, 100) {
            Ok(Some(budgets)) => Ok(Custom(Status::Ok, json!(budgets))),
            Ok(None) => Err(Custom(
                Status::NotFound,
                json!({"error": "Budget not found"}),
            )),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                json!({"error":"something went wrong"}),
            )),
        },
    )
    .await
}

#[put("/budget/<id>", format = "json", data = "<budget>")]
pub async fn update_budget(
    db: DBConnection,
    id: i32,
    _auth: AuthenticatedUser,
    budget: Json<UpdateBudget>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| match BudgetRepository::find_budget(c, id) {
        Ok(Some(_)) => match budget.validate() {
            Ok(()) => match BudgetRepository::update_budget(c, id, budget.into_inner()) {
                Ok(budget_res) => Ok(Custom(Status::Created, json!({"message":budget_res}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
            },
            Err(errors) => Err(Custom(Status::BadRequest, json!({"error":errors}))),
        },
        Ok(None) => Err(Custom(
            Status::NotFound,
            json!({"error": "Budget not found"}),
        )),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            json!({"error":"Something went wrong"}),
        )),
    })
    .await
}

#[delete("/budget/<id>")]
pub async fn delete_budget(
    db: DBConnection,
    _auth: AuthenticatedUser,
    id: i32,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| match BudgetRepository::find_budget(c, id) {
        Ok(Some(_)) => match BudgetRepository::delete_budget(c, id) {
            Ok(_) => Ok(json!({"message":"budget deleted successfully"})),
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
            json!({"error": "Budget not found"}),
        )),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            json!({"error":"Something went wrong"}),
        )),
    })
    .await
}
