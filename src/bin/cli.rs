extern crate env_logger;
extern crate finance_manager;

use clap::{Arg, Command};
use finance_manager::command::commands::{create_user, list_users};

fn main() {
    env_logger::init();

    let matches = Command::new("user_registration")
        .version("1.0")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User commands")
                .subcommand(
                    Command::new("create")
                        .about("Create a new user")
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("email").required(true))
                        .arg(Arg::new("password").required(true)),
                )
                .subcommand(Command::new("list").about("list existing users")),
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
        }
        if let Some(("list", _)) = sub_matches.subcommand() {
            list_users()
        }
    }
}
