use crate::schema::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use validator_derive::Validate;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Insertable, Deserialize, Validate)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 3, message = "Username should be more than 2 characters"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password_hash: String,
}

#[derive(Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
pub struct Budget {
    pub id: i32,
    pub user_id: Option<i32>,
    pub category: String,
    pub amount: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = budgets)]
pub struct NewBudget {
    pub user_id: Option<i32>,
    pub category: String,
    pub amount: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub user_id: i32,
    pub amount: BigDecimal,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
}

#[derive(Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub amount: BigDecimal,
    pub category: String,
    pub description: String,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginCredentials {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password_hash: String,
}
