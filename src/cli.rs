use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("Personal Finance Manager")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manage your finances")
        .subcommand(
            Command::new("add")
                .about("Add a new transaction")
                .arg(
                    Arg::new("type")
                        .long("type")
                        .required(true)
                        .value_parser(clap::value_parser!(String))
                        .help("Transaction type (income/expense)"),
                )
                .arg(
                    Arg::new("amount")
                        .long("amount")
                        .required(true)
                        .value_parser(clap::value_parser!(f64)) // Ensure amount is parsed as f64
                        .help("Transaction amount"),
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .value_parser(clap::value_parser!(String))
                        .help("Transaction description"),
                ),
        )
        .subcommand(Command::new("list").about("List all transactions"))
        .subcommand(Command::new("balance").about("Calculate the current account balance"))
        .subcommand(
            Command::new("delete")
                .about("Delete a transaction")
                .arg(
                    Arg::new("id")
                        .long("id")
                        .required(true)
                        .value_parser(clap::value_parser!(i32)) // Ensure id is parsed as i32
                        .help("Transaction ID to delete"),
                ),
        )
}
