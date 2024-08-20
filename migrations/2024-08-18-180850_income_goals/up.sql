-- Your SQL goes here
CREATE TABLE goals (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    goal_description TEXT NOT NULL,
    goal_amount DECIMAL NOT NULL,
    deadline DATE NOT NULL
);

CREATE TABLE income (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    amount DECIMAL NOT NULL,
    source TEXT NOT NULL,
    date DATE NOT NULL
);