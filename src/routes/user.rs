use crate::model::{LoginCredentials, NewUser};
use crate::repositories::UserRepository;
use crate::utils::hashing::{hash_password, verify_password};
use crate::utils::jwt_token::generate_jwt;
use crate::{AuthenticatedUser, DBConnection};
use rocket::put;
use rocket::{
    http::Status,
    post,
    response::status::Custom,
    serde::json::{serde_json::json, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    user: UserResponse,
    token: String,
}

#[post("/user/register", format = "json", data = "<new_user>")]
pub async fn create_user(
    db: DBConnection,
    new_user: Json<NewUser>,
) -> Result<Custom<Value>, Custom<Value>> {
    let mut user = new_user.into_inner();

    // Validate the user data
    match user.validate() {
        Ok(()) => {
            match hash_password(&user.password_hash) {
                Ok(hashed_password) => user.password_hash = hashed_password,
                Err(err) => {
                    eprintln!("Error hashing password: {:?}", err);
                    return Err(Custom(
                        Status::InternalServerError,
                        json!({"error":"Failed to hash password"}),
                    ));
                }
            }
            // Data is valid, proceed with creating the user
            db.run(move |c| {
                if UserRepository::find_by_email(c, &user.email).is_ok() {
                    return Err(Custom(
                        Status::Conflict,
                        json!({"message":"User with this email already exists"}),
                    ));
                }
                match UserRepository::create_record(c, user) {
                    Ok(a_user) => Ok(Custom(Status::Created, json!(a_user))),
                    Err(err) => {
                        eprintln!("Error creating a user: {:?}", err);
                        Err(Custom(
                            Status::InternalServerError,
                            json!({"error":"Something went wrong"}),
                        ))
                    }
                }
            })
            .await
        }
        Err(errors) => {
            // Validation failed, return an error response
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .map(|(field, error)| format!("{}: {:?}", field, error))
                .collect();
            Err(Custom(
                Status::BadRequest,
                json!({"errors": error_messages}),
            ))
        }
    }
}

#[post("/user/login", format = "json", data = "<login_credentials>")]
pub async fn login(
    db: DBConnection,
    login_credentials: Json<LoginCredentials>,
) -> Result<Custom<Value>, Custom<Value>> {
    let email = login_credentials.email.clone();
    let password = login_credentials.password_hash.clone();

    let result = db
        .run(move |c| UserRepository::find_by_email(c, &email))
        .await;
    match login_credentials.validate() {
        Ok(()) => match result {
            Ok(Some(user)) => match verify_password(&user.password_hash, &password) {
                Ok(_) => match generate_jwt(&user.email, &user.id) {
                    Ok(token) => Ok(Custom(
                        Status::Ok,
                        json!(CreateUserResponse {
                            user: UserResponse {
                                id: user.id,
                                username: user.username,
                                email: user.email,
                            },
                            token,
                        }),
                    )),
                    Err(_) => Err(Custom(
                        Status::InternalServerError,
                        json!("Failed to generate token"),
                    )),
                },
                Err(_) => Err(Custom(
                    Status::Forbidden,
                    json!({"error": "Wrong credentials"}),
                )),
            },
            Ok(None) => Err(Custom(
                Status::Forbidden,
                json!({"error": "Wrong credentials"}),
            )),
            Err(err) => {
                eprintln!("Error finding user: {:?}", err);
                Err(Custom(
                    Status::InternalServerError,
                    json!({"error":"Something went wrong"}),
                ))
            }
        },
        Err(errors) => {
            // Validation failed, return an error response
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .map(|(field, error)| format!("{}: {:?}", field, error))
                .collect();
            Err(Custom(
                Status::BadRequest,
                json!({"errors": error_messages}),
            ))
        }
    }
}

#[put("/user/<id>", format = "json", data = "<user>")]
pub async fn update_user(
    db: DBConnection,
    id: i32,
    _auth: AuthenticatedUser,
    user: Json<NewUser>,
) -> Result<Custom<Value>, Custom<Value>> {
    let mut user = user.into_inner();
    db.run(move |c| {
        match hash_password(&user.password_hash) {
            Ok(hashed_password) => user.password_hash = hashed_password,
            Err(err) => {
                eprintln!("Error hashing password: {:?}", err);
                return Err(Custom(
                    Status::InternalServerError,
                    json!({"error":"Failed to hash password"}),
                ));
            }
        }

        match UserRepository::find_by_email(c, &user.email) {
            Ok(Some(_)) => match user.validate() {
                Ok(()) => match UserRepository::update_user(c, id, user) {
                    Ok(budget_res) => Ok(Custom(Status::Ok, json!({"message":budget_res}))),
                    Err(_) => Err(Custom(Status::InternalServerError, json!("error"))),
                },
                Err(errors) => Err(Custom(Status::BadRequest, json!({"error":errors}))),
            },
            Ok(None) => Err(Custom(Status::NotFound, json!({"error": "User not found"}))),
            Err(_) => Err(Custom(
                Status::InternalServerError,
                json!({"error":"Something went wrong"}),
            )),
        }
    })
    .await
}
