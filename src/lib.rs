#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
extern crate diesel_migrations;

mod model;
pub mod repositories;
pub mod routes;
pub mod schema;
pub mod utils;

use crate::utils::jwt_token::{validate_jwt, Claims};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct DBConnection(PgConnection);

pub struct AuthenticatedUser {
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers = request.headers();
        if let Some(auth_header) = headers.get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                match validate_jwt(token) {
                    Ok(claims) => {
                        return Outcome::Success(AuthenticatedUser {
                            email: claims.email,
                        });
                    }
                    Err(_) => return Outcome::Error((Status::Unauthorized, ())),
                }
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}
