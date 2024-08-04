use crate::command::errors::AppError;
use crate::model::{NewUser, UserResponse};
use crate::repositories::UserRepository;
use crate::utils::hashing::hash_password;
use diesel::{Connection, PgConnection};

fn load_db_connection() -> Result<PgConnection, AppError> {
    let database_url = dotenv::var("DATABASE_URL").expect("cannot load database url from env");
    Ok(PgConnection::establish(&database_url)?)
}

pub fn create_user(username: String, email: String, password_hash: String) -> Result<(), AppError> {
    let mut connection = load_db_connection()?;
    let email_clone = email.clone();

    let hashed_password = hash_password(&password_hash).unwrap();
    let new_user: NewUser = NewUser {
        username,
        email: email_clone,
        password_hash: hashed_password,
    };

    if UserRepository::find_by_email(&mut connection, email).is_ok() {
        return Err(AppError::UserRepositoryError);
    };

    match UserRepository::create_record(&mut connection, new_user) {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::UserRepositoryError),
    }
}

pub fn list_users() {
    let mut c = load_db_connection().unwrap();
    let users = UserRepository::find_multiple_users(&mut c, 100).expect("Error fetching users");
    for user in users {
        let res = UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        };
        println!("{:?}", res);
    }
}

pub fn delete_users(id: i32) {
    let mut c = load_db_connection().unwrap();
    let _ = UserRepository::delete_record(&mut c, id).expect("user deletion failed");
    println!("Deleted user with id: {:?}", id);
}

pub fn update_user(
    current_email: String,
    username: String,
    email: String,
    password_hash: String,
) -> Result<(), AppError> {
    let mut connection = load_db_connection()?;
    let email_clone = email.clone();

    let hashed_password = hash_password(&password_hash).unwrap();
    let new_user: NewUser = NewUser {
        username,
        email: email_clone,
        password_hash: hashed_password,
    };

    match UserRepository::find_by_email(&mut connection, current_email) {
        Ok(Some(user)) => match UserRepository::update_user(&mut connection, user.id, new_user) {
            Ok(_) => Ok(()),
            Err(_) => Err(AppError::UserRepositoryError),
        },
        Ok(None) => Err(AppError::UserRepositoryError),
        Err(_) => Err(AppError::UserRepositoryError),
    }
}
