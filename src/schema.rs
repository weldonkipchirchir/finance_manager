// @generated automatically by Diesel CLI.

diesel::table! {
    budgets (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        category -> Text,
        amount -> Numeric,
        start_date -> Date,
        end_date -> Date,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        amount -> Numeric,
        category -> Text,
        description -> Nullable<Text>,
        date -> Date,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

diesel::joinable!(budgets -> users (user_id));
diesel::joinable!(transactions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(budgets, transactions, users,);
