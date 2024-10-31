mod extract;
mod load_transform;
mod query;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("extract") => {
            if let Err(e) = extract::extract_data() {
                eprintln!("Error extracting data: {}", e);
            }
        }
        Some("load") => {
            if let Err(e) = load_transform::load_data() {
                eprintln!("Error loading data: {}", e);
            }
        }
        Some("query") => {
            if let Err(e) = query::run_query() {
                eprintln!("Error running query: {}", e);
            }
        }
        None => {
            eprintln!("Please provide a command: extract, load, or query");
        }
        _ => {
            eprintln!("Unknown command");
        }
    };
}
