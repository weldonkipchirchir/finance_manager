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
    pub fn find_by_email(c: &mut PgConnection, email: &String) -> QueryResult<Option<User>> {
        diesel::QueryDsl::filter(users::table, users::email.eq(email))
            .get_result::<User>(c)
            .optional()
    }
    pub fn find_multiple_users(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).load::<User>(c)
    }
    pub fn delete_record(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c)
    }
    pub fn update_user(
        c: &mut PgConnection,
        id: i32,
        update: NewUser,
    ) -> QueryResult<Option<User>> {
        diesel::update(users::table.find(id))
            .set((
                users::username.eq(update.username.to_owned()),
                users::email.eq(update.email.to_owned()),
                users::password_hash.eq(update.password_hash.to_owned()),
            ))
            .execute(c)?;
        Self::find_by_email(c, &update.email)
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

pub struct IncomeRepository;
impl IncomeRepository {
    pub fn create_income(c: &mut PgConnection, record: NewIncome) -> QueryResult<Income> {
        diesel::insert_into(income::table)
            .values(record)
            .get_result(c)
    }
    pub fn find_multiple_income(
        c: &mut PgConnection,
        limit: i64,
    ) -> QueryResult<Option<Vec<Income>>> {
        income::table.limit(limit).load::<Income>(c).optional()
    }
    pub fn find_income(c: &mut PgConnection, id: i32) -> QueryResult<Option<Income>> {
        income::table.find(id).get_result::<Income>(c).optional()
    }
    pub fn delete_income(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(income::table.find(id)).execute(c)
    }
    pub fn update_income(
        c: &mut PgConnection,
        id: i32,
        update: NewIncome    
    )-> QueryResult<Option<Income>>{
        diesel::update(income::table.find(id))
            .set((
                income::amount.eq(update.amount),
                income::source.eq(update.source),
                income::date.eq(update.date),
            ))
            .execute(c)?;
        Self::find_income(c, id)
    }
}

pub struct GoalsRepository;
impl GoalsRepository {
    pub fn create_goal(c: &mut PgConnection, record: NewGoal) -> QueryResult<Goals> {
        diesel::insert_into(goals::table)
            .values(record)
            .get_result(c)
    }
    pub fn find_multiple_goals(
        c: &mut PgConnection,
        limit: i64,
    ) -> QueryResult<Option<Vec<Goals>>> {
        goals::table.limit(limit).load::<Goals>(c).optional()
    }
    pub fn find_goal(c: &mut PgConnection, id: i32) -> QueryResult<Option<Goals>> {
        goals::table.find(id).get_result::<Goals>(c).optional()
    }
    pub fn delete_goal(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(goals::table.find(id)).execute(c)
    }
    pub fn update_goal(
        c: &mut PgConnection,
        id: i32,
        record: NewGoal,
    )-> QueryResult<Option<Goals>>{
        diesel::update(goals::table.find(id))
            .set((
                goals::goal_amount.eq(record.goal_amount),
                goals::goal_description.eq(record.goal_description),
                goals::deadline.eq(record.deadline),
                goals::saving.eq(record.saving),
            ))
            .execute(c)?;
        Self::find_goal(c, id)
    }
}
