use crate::model::*;
use crate::schema::*;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

pub struct UserRepository;

impl UserRepository {
    pub fn create_record(c: &mut PgConnection, record: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(record)
            .get_result(c)
    }
    pub fn find_by_email(c: &mut PgConnection, email: &String) -> QueryResult<User> {
        diesel::QueryDsl::filter(users::table, users::email.eq(email)).get_result::<User>(c)
    }
    pub fn find_multiple_users(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table
            .limit(limit)
            .order(users::id.desc())
            .load::<User>(c)
    }
    pub fn delete_record(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c)
    }
}
