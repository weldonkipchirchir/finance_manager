use crate::schema::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use validator::{Validate as ValidatorValidate, ValidationError};
use validator_derive::Validate;

fn validate_positive_amount(amount: &BigDecimal) -> Result<(), ValidationError> {
    if amount <= &BigDecimal::from(0) {
        return Err(ValidationError::new("amount must be positive"));
    }
    Ok(())
}

fn validate_start_date_before_end_date(
    start_date: &NaiveDate,
    end_date: &NaiveDate,
) -> Result<(), ValidationError> {
    if start_date > end_date {
        return Err(ValidationError::new("start_date must be before end_date"));
    }
    Ok(())
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
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

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Queryable, Associations, Serialize, Deserialize, Validate)]
#[diesel(belongs_to(User))]
pub struct Budget {
    pub id: i32,
    pub user_id: Option<i32>,
    #[validate(length(min = 3, message = "Category should be more than 2 characters"))]
    pub category: String,
    pub amount: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = budgets)]
pub struct NewBudget {
    pub user_id: Option<i32>,
    #[validate(length(min = 3, message = "Category should be more than 2 characters"))]
    pub category: String,
    pub amount: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl NewBudget {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate positive amount
        validate_positive_amount(&self.amount)?;

        // Validate date range
        validate_start_date_before_end_date(&self.start_date, &self.end_date)?;

        Ok(())
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = budgets)]
pub struct UpdateBudget {
    #[validate(length(min = 3, message = "Category should be more than 2 characters"))]
    pub category: String,
    pub amount: BigDecimal,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl UpdateBudget {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate positive amount
        validate_positive_amount(&self.amount)?;

        // Validate date range
        validate_start_date_before_end_date(&self.start_date, &self.end_date)?;

        Ok(())
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub user_id: Option<i32>,
    pub amount: BigDecimal,
    #[validate(length(min = 3, message = "Category should be more than 2 characters"))]
    pub category: String,
    #[validate(length(min = 3, message = "Description should be more than 2 characters"))]
    pub description: Option<String>,
    pub date: NaiveDate,
}

impl NewTransaction {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate positive amount
        validate_positive_amount(&self.amount)?;

        Ok(())
    }
}

#[derive(Insertable, Serialize, Deserialize, Validate)]
#[diesel(table_name = transactions)]
pub struct UpdateTransaction {
    pub amount: BigDecimal,
    #[validate(length(min = 3, message = "Category should be more than 2 characters"))]
    pub category: String,
    #[validate(length(min = 3, message = "Description should be more than 2 characters"))]
    pub description: Option<String>,
    pub date: NaiveDate,
}

impl UpdateTransaction {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Validate positive amount
        validate_positive_amount(&self.amount)?;

        Ok(())
    }
}

#[derive(Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
pub struct Transaction {
    pub id: i32,
    pub user_id: Option<i32>,
    pub amount: BigDecimal,
    pub category: String,
    pub description: Option<String>,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, ValidatorValidate)]
pub struct LoginCredentials {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password_hash: String,
}
