use crate::model::NewIncome;
use crate::repositories::IncomeRepository;
use crate::{AuthenticatedUser, DBConnection};
use rocket::{delete, get, put};
use rocket::{
    http::Status,
    post,
    response::status::Custom,
    serde::json::{serde_json::json, Json},
};
use serde_json::Value;
#[post("/income", format = "json", data = "<new_income>")]
pub async fn create_income(
    db: DBConnection,
    new_income: Json<NewIncome>,
    auth: AuthenticatedUser,
) -> Result<Custom<Value>, Custom<Value>> {
    let mut income = new_income.into_inner();
    income.user_id = Some(auth.id);

    match income.validate() {
        Ok(()) => {
            db.run(move |c| match IncomeRepository::create_income(c, income) {
                Ok(income_res) => Ok(Custom(Status::Created, json!({"message":income_res}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
            })
            .await
        }
        Err(error) => Err(Custom(Status::BadRequest, json!({"errors": error}))),
    }
}

#[get("/incomes")]
pub async fn view_incomes(
    db: DBConnection,
    _auth: AuthenticatedUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(
        move |c| match IncomeRepository::find_multiple_income(c, 100) {
            Ok(Some(incomes)) => Ok(Custom(Status::Ok, json!(incomes))),
            Ok(None) => Ok(Custom(Status::NotFound, json!("Not found"))),
            Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
        },
    )
    .await
}

#[get("/income/<id>")]
pub async fn view_income(
    db: DBConnection,
    _auth: AuthenticatedUser,
    id: i32
)-> Result<Custom<Value>, Custom<Value>>{
    db.run(move |c| match IncomeRepository::find_income(c, id) {
        Ok(Some(income))=> Ok(Custom(Status::Ok, json!(income))),
        Ok(None)=> Err(Custom(Status::NotFound, json!({"error":"income not found"}))),
        Err(_)=> Err(Custom(Status::InternalServerError, json!({"error":"something went wrong"})))
    }).await
}

#[put("/income/<id>", format="json", data="<income>")]
pub async fn update_income(
    db: DBConnection,
    id: i32,
    income: Json<NewIncome>,
    _auth: AuthenticatedUser,
) ->Result<Custom<Value>, Custom<Value>>{
    let income = income.into_inner();

    db.run(move |c| match IncomeRepository::find_income(c, id){
        Ok(Some(_))=> match  income.validate() {
            Ok(())=> match  IncomeRepository::update_income(c, id, income) {
                Ok(income_res)=> Ok(Custom(Status::Ok, json!({"message":income_res}))),
                Err(_)=> Err(Custom(Status::InternalServerError, json!({"error":"something went wrong"})))
            },
            Err(errors) => Err(Custom(Status::BadRequest, json!({"error":errors}))),
        }
        Ok(None) => Err(Custom(
            Status::NotFound,
            json!({"error": "Income not found"}),
        )),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            json!({"error":"Something went wrong"}),
        )),
    }).await
}


#[delete("/income/<id>")]
pub async fn delete_income(
    db: DBConnection,
    id: i32,
    _auth: AuthenticatedUser,
    ) -> Result<Custom<Value>, Custom<Value>> {
        db.run(move |c| match IncomeRepository::find_income(c, id) {
            Ok(Some(_)) => match IncomeRepository::delete_income(c, id) {
                Ok(_) => Ok(Custom(Status::Ok, json!({"message": "Income deleted"}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!({"error":"something went wrong"}))),
            },
                Ok(None) => Err(Custom(Status::NotFound, json!({"error":"income not found"}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!({"error":"something went wrong"}))),
            })
        .await
}
