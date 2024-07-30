# Finance Manager Project

This `Finance Manager` project is a web application for managing personal finances. It allows users to register, log in, create budgets, and track transactions. The backend is built using Rust with Rocket and Diesel, and it includes user authentication and JWT-based authorization.

## Table of Contents

- [Features](#features)
- [Dependencies](#dependencies)
- [Setup](#setup)
- [Running the Application](#running-the-application)
- [API Endpoints](#api-endpoints)
- [Environment Variables](#environment-variables)

## Features

- User Registration
- User Login
- JWT-based Authentication
- Create, Read, Update, Delete (CRUD) operations for Budgets and Transactions
- Secure password hashing
- Validation of user inputs

## Dependencies

- **[Rocket](https://rocket.rs/)**: Web framework for Rust.
- **[Diesel](https://diesel.rs/)**: ORM for interacting with the database.
- **[Bcrypt](https://crates.io/crates/bcrypt)**: Library for hashing passwords.
- **[JSON Web Token (JWT)](https://crates.io/crates/jsonwebtoken)**: Library for creating and verifying JWTs.
- **[Serde](https://serde.rs/)**: Framework for serializing and deserializing Rust data structures.
- **[Validator](https://crates.io/crates/validator)**: Library for validating data structures.

## Setup

1. **Clone the repository**

    ```sh
    git clone https://github.com/weldonkipchirchir/finance_manager.git
    cd finance_manager
    ```

2. **Install Rust and Cargo**

    Follow the instructions at [rustup.rs](https://rustup.rs/) to install Rust and Cargo.

3. **Install Diesel CLI**

    ```sh
    cargo install diesel_cli --no-default-features --features postgres
    ```

4. **Setup PostgreSQL Database**

    Create a PostgreSQL database and set the connection URL in the `.env` file.

5. **Run Database Migrations**

    ```sh
    diesel setup
    diesel migration run
    ```

## Running the Application

1. **Start the Application**

    ```sh
    cargo run --bin server
    ```

2. **Access the API**

    The API will be available at `http://localhost:8000`.

## API Endpoints

### Authentication

- **POST /user/register**
  
  Register a new user.
  
  **Request:**
  ```json
  {
    "username": "johndoe",
    "email": "johndoe@example.com",
    "password": "password123"
  }
  ```

  **Response:**
  ```json
  {
    "user": {
      "id": 1,
      "username": "johndoe",
      "email": "johndoe@example.com"
    },
    "token": "jwt_token"
  }
  ```

- **POST /user/login**

  Log in an existing user.
  
  **Request:**
  ```json
  {
    "email": "johndoe@example.com",
    "password": "password123"
  }
  ```

  **Response:**
  ```json
  {
    "user": {
      "id": 1,
      "username": "johndoe",
      "email": "johndoe@example.com"
    },
    "token": "jwt_token"
  }
  ```

### Budgets

- **GET /budgets**

  Get all budgets for the authenticated user.

- **POST /budgets**

  Create a new budget.

  **Request:**
  ```json
  {
    "category": "Groceries",
    "amount": 200.00,
    "start_date": "2023-01-01",
    "end_date": "2023-01-31"
  }
  ```

  **Response:**
  ```json
  {
    "id": 1,
    "user_id": 1,
    "category": "Groceries",
    "amount": 200.00,
    "start_date": "2023-01-01",
    "end_date": "2023-01-31"
  }
  ```

- **PUT /budgets/:id**

  Update an existing budget.

- **DELETE /budgets/:id**

  Delete a budget.

### Transactions

- **GET /transactions**

  Get all transactions for the authenticated user.

- **POST /transactions**

  Create a new transaction.

  **Request:**
  ```json
  {
    "amount": 50.00,
    "category": "Groceries",
    "description": "Weekly grocery shopping",
    "date": "2023-01-05"
  }
  ```

  **Response:**
  ```json
  {
    "id": 1,
    "user_id": 1,
    "amount": 50.00,
    "category": "Groceries",
    "description": "Weekly grocery shopping",
    "date": "2023-01-05"
  }
  ```

- **PUT /transactions/:id**

  Update an existing transaction.

- **DELETE /transactions/:id**

  Delete a transaction.

## Environment Variables

Create a `.env` file in the project root and add the following environment variables:

```
DATABASE_URL=postgres://username:password@localhost/finance_manager
JWT_SECRET=your_jwt_secret
```

Replace `username`, `password`, and `your_jwt_secret` with your actual PostgreSQL username, password, and desired JWT secret.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
