extern crate env_logger;
extern crate finance_manager;

use clap::{Arg, Command};
use finance_manager::command::commands::{create_user, delete_users, list_users, update_user};

fn main() {
    env_logger::init();

    let matches = Command::new("user_registration")
        .version("1.0")
        .about("User Registration CLI")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg(
                            Arg::new("username")
                                .help("Username of the new user")
                                .required(true),
                        )
                        .arg(
                            Arg::new("email")
                                .help("Email of the new user")
                                .required(true),
                        )
                        .arg(
                            Arg::new("password")
                                .help("Password for the new user")
                                .required(true),
                        ),
                )
                .subcommand(Command::new("list").about("List existing users"))
                .subcommand(
                    Command::new("delete").about("Delete an existing user").arg(
                        Arg::new("id")
                            .help("ID of the user to delete")
                            .required(true)
                            .value_parser(clap::value_parser!(i32)),
                    ),
                )
                .subcommand(
                    Command::new("update")
                        .about("Update an existing user")
                        .arg(
                            Arg::new("current_email")
                                .help("Current Email")
                                .required(true),
                        )
                        .arg(Arg::new("username").help("New Username").required(true))
                        .arg(Arg::new("email").help("New Email").required(true))
                        .arg(Arg::new("password").help("New Password").required(true)),
                ),
        )
        .get_matches();

    if let Some(("users", sub_matches)) = matches.subcommand() {
        if let Some(("create", matches)) = sub_matches.subcommand() {
            let username = matches.get_one::<String>("username").unwrap().to_owned();
            let email = matches.get_one::<String>("email").unwrap().to_owned();
            let password = matches.get_one::<String>("password").unwrap().to_owned();

            match create_user(username, email, password) {
                Ok(_) => println!("User created successfully."),
                Err(e) => eprintln!("Error creating user: {:?}", e),
            }
        } else if let Some(("list", _)) = sub_matches.subcommand() {
            list_users()
        } else if let Some(("delete", matches)) = sub_matches.subcommand() {
            let id = matches.get_one::<i32>("id").unwrap();
            delete_users(*id);
        } else if let Some(("update", matches)) = sub_matches.subcommand() {
            let current_email = matches
                .get_one::<String>("current_email")
                .unwrap()
                .to_owned();
            let username = matches.get_one::<String>("username").unwrap().to_owned();
            let email = matches.get_one::<String>("email").unwrap().to_owned();
            let password = matches.get_one::<String>("password").unwrap().to_owned();

            match update_user(current_email, username, email, password) {
                Ok(_) => println!("User updated successfully."),
                Err(e) => eprintln!("Error updating user: {:?}", e),
            }
        }
    }
}
