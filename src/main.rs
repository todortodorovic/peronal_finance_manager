mod database;
mod cli; // Include the cli module

use database::{add_transaction, delete_transaction, initialize_database, list_transactions,calculate_balance};

fn main() {
    let conn = initialize_database().expect("Failed to initialize database");

    let cli = cli::build_cli().get_matches(); // Use the build_cli function from cli.rs

    match cli.subcommand() {
        Some(("add", sub_m)) => {
            let trans_type = sub_m
                .get_one::<String>("type")
                .expect("Transaction type is required");
            let amount = *sub_m
                .get_one::<f64>("amount")
                .expect("Transaction amount is required");
            let description = sub_m
                .get_one::<String>("description")
                .map(String::as_str);

            if let Err(e) = add_transaction(&conn, trans_type, amount, description) {
                eprintln!("Error adding transaction: {}", e);
            } else {
                println!("Transaction added: {} - {}", trans_type, amount);
            }
        }

        Some(("list", _)) => {
            match list_transactions(&conn) {
                Ok(transactions) => {
                    for (id, trans_type, amount, description, date) in transactions {
                        println!(
                            "{}: {} - {} - {} ({})",
                            id, trans_type, amount, description, date
                        );
                    }
                }
                Err(e) => eprintln!("Error listing transactions: {}", e),
            }
        }

        Some(("delete", sub_m)) => {
            let id = *sub_m
                .get_one::<i32>("id")
                .expect("Transaction ID is required");

            if let Err(e) = delete_transaction(&conn, id) {
                eprintln!("Error deleting transaction: {}", e);
            } else {
                println!("Transaction deleted with ID: {}", id);
            }
        }
        Some(("balance", _)) => {
            match calculate_balance(&conn) {
                Ok(balance) => println!("Current account balance: {:.2}", balance),
                Err(e) => eprintln!("Error calculating balance: {}", e),
            }
        }
        

        _ => println!("Unknown command. Use --help for more information."),
    }
}
