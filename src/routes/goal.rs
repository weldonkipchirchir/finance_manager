use crate::model::NewGoal;
use crate::repositories::GoalsRepository;
use crate::{AuthenticatedUser, DBConnection};
use rocket::{delete, get};
use rocket::{
    http::Status,
    post,
    put,
    response::status::Custom,
    serde::json::{serde_json::json, Json},
};
use serde_json::Value;
#[post("/goal", format = "json", data = "<new_goal>")]
pub async fn create_goal(
    db: DBConnection,
    new_goal: Json<NewGoal>,
    auth: AuthenticatedUser,
) -> Result<Custom<Value>, Custom<Value>> {
    let mut goal = new_goal.into_inner();
    goal.user_id = Some(auth.id);

    match goal.validate() {
        Ok(()) => {
            db.run(move |c| match GoalsRepository::create_goal(c, goal) {
                Ok(goal_res) => Ok(Custom(Status::Created, json!({"message":goal_res}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
            })
            .await
        }
        Err(error) => Err(Custom(Status::BadRequest, json!({"errors": error}))),
    }
}

#[get("/goals")]
pub async fn view_goals(
    db: DBConnection,
    _auth: AuthenticatedUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(
        move |c| match GoalsRepository::find_multiple_goals(c, 100) {
            Ok(Some(goals)) => Ok(Custom(Status::Ok, json!(goals))),
            Ok(None) => Ok(Custom(Status::NotFound, json!("Not found"))),
            Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
        },
    )
    .await
}

#[get("/goal/<id>")]
pub async fn view_goal(
    db: DBConnection,
    _auth: AuthenticatedUser,
    id: i32
)-> Result<Custom<Value>, Custom<Value>>{
    db.run(move |c| match GoalsRepository::find_goal(c, id) {
        Ok(Some(goal))=> Ok(Custom(Status::Ok, json!(goal))),
        Ok(None)=> Err(Custom(Status::NotFound, json!({"error":"goal not found"}))),
        Err(_)=> Err(Custom(Status::InternalServerError, json!({"error":"something went wrong"})))
    }).await
}

#[put("/goal/<id>", format="json", data="<goal>")]
pub async fn update_goal(
    db: DBConnection,
    id: i32,
    goal: Json<NewGoal>,
    _auth: AuthenticatedUser,
) -> Result<Custom<Value>, Custom<Value>> {
    let goal = goal.into_inner();

    db.run(move |c| match GoalsRepository::find_goal(c, id) {
        Ok(Some(_)) => match goal.validate() {
            Ok(()) => match GoalsRepository::update_goal(c, id, goal) {
                Ok(goal_res) => Ok(Custom(Status::Ok, json!({"message": goal_res}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!({"error": "something went wrong"})))
            },
            Err(errors) => Err(Custom(Status::BadRequest, json!({"error": errors}))),
        },
        Ok(None) => Err(Custom(
            Status::NotFound,
            json!({"error": "Goal not found"}),
        )),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            json!({"error": "Something went wrong"}),
        )),
    }).await
}


#[delete("/goal/<id>")]
pub async fn delete_goal(
    db: DBConnection,
    id: i32,
    _auth: AuthenticatedUser,
    ) -> Result<Custom<Value>, Custom<Value>> {
        db.run(move |c| match GoalsRepository::find_goal(c, id) {
            Ok(Some(_)) => match GoalsRepository::delete_goal(c, id) {
                Ok(_) => Ok(Custom(Status::Ok, json!({"message": "Goal deleted"}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!({"error":"something went wrong"}))),
            },
                Ok(None) => Err(Custom(Status::NotFound, json!({"error":"goal not found"}))),
                Err(_) => Err(Custom(Status::InternalServerError, json!({"error":"something went wrong"}))),
            })
        .await
}
