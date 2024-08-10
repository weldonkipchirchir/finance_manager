use crate::model::*;
use crate::schema::*;
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
        users::table.limit(limit).load::<User>(c)
    }
    pub fn delete_record(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c)
    }
}

pub struct BudgetRepository;

impl BudgetRepository {
    pub fn create_budget(c: &mut PgConnection, record: NewBudget) -> QueryResult<Budget> {
        diesel::insert_into(budgets::table)
            .values(record)
            .get_result(c)
    }
    pub fn find_multiple_budgets(
        c: &mut PgConnection,
        limit: i64,
    ) -> QueryResult<Option<Vec<Budget>>> {
        budgets::table.limit(limit).load::<Budget>(c).optional()
    }
    pub fn find_budget(c: &mut PgConnection, id: i32) -> QueryResult<Option<Budget>> {
        budgets::table.find(id).get_result::<Budget>(c).optional()
    }
    pub fn update_budget(
        c: &mut PgConnection,
        id: i32,
        update: UpdateBudget,
    ) -> QueryResult<Option<Budget>> {
        diesel::update(budgets::table.find(id))
            .set((
                budgets::amount.eq(update.amount.to_owned()),
                budgets::category.eq(update.category.to_owned()),
                budgets::start_date.eq(update.start_date.to_owned()),
                budgets::end_date.eq(update.end_date.to_owned()),
            ))
            .execute(c)?;
        Self::find_budget(c, id)
    }
    pub fn delete_budget(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(budgets::table.find(id)).execute(c)
    }
}

pub struct TransactionsRepository;

impl TransactionsRepository {
    pub fn create_transaction(
        c: &mut PgConnection,
        record: NewTransaction,
    ) -> QueryResult<Transaction> {
        diesel::insert_into(transactions::table)
            .values(record)
            .get_result(c)
    }
    pub fn find_multiple_transactions(
        c: &mut PgConnection,
        limit: i64,
    ) -> QueryResult<Option<Vec<Transaction>>> {
        transactions::table
            .limit(limit)
            .load::<Transaction>(c)
            .optional()
    }
    pub fn find_transaction(c: &mut PgConnection, id: i32) -> QueryResult<Option<Transaction>> {
        transactions::table
            .find(id)
            .get_result::<Transaction>(c)
            .optional()
    }
    pub fn update_transaction(
        c: &mut PgConnection,
        id: i32,
        update: UpdateTransaction,
    ) -> QueryResult<Option<Transaction>> {
        diesel::update(transactions::table.find(id))
            .set((
                transactions::amount.eq(update.amount.to_owned()),
                transactions::description.eq(update.description.to_owned()),
                transactions::category.eq(update.category.to_owned()),
                transactions::date.eq(update.date.to_owned()),
            ))
            .execute(c)?;
        Self::find_transaction(c, id)
    }
    pub fn delete_transaction(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(transactions::table.find(id)).execute(c)
    }
}
