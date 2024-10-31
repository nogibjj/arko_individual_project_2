use rusqlite::{Connection, Result};
use std::io::{self, Write};

pub fn run_query() -> Result<String, std::io::Error> {
    let conn = Connection::open("aapl.db").expect("Failed to open database");

    println!("Enter your SQL query or type 'default' for the default query:");

    let mut user_query = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_query)
        .expect("Failed to read line");

    let query_to_run = if user_query.trim().eq_ignore_ascii_case("default") {
        r#"WITH RecentData AS (
  SELECT
    close AS ClosePrice,
    date,
    LAG(close, 1) OVER (ORDER BY date DESC) AS PrevClosePrice
  FROM AAPL
  ORDER BY date DESC
  LIMIT 6  -- Get 6 rows to account for 5 changes
)
SELECT
  date,
  ClosePrice,
  PrevClosePrice,
  ROUND(((ClosePrice - PrevClosePrice) / PrevClosePrice) * 100, 2) AS PercentChange
FROM RecentData
WHERE PrevClosePrice IS NOT NULL
ORDER BY date DESC;"#
    } else {
        user_query.trim()
    };

    // Prepare the statement and handle potential errors
    match conn.prepare(query_to_run) {
        Ok(mut stmt) => {
            let stocks_iter = stmt.query_map([], |row| {
                let date: String = row.get(0).unwrap_or_default();
                let close_price: f64 = row.get(1).unwrap_or_default();
                let prev_close_price: f64 = row.get(2).unwrap_or_default();
                let percent_change: f64 = row.get(3).unwrap_or_default();

                // Create a formatted string for the output
                Ok(format!(
                    "Date: {}, ClosePrice: {}, PrevClosePrice: {}, PercentChange: {}",
                    date, close_price, prev_close_price, percent_change
                ))
            });

            match stocks_iter {
                Ok(iter) => {
                    let mut output_found = false; // Flag to check if any output is printed
                    for stock in iter {
                        if let Ok(output) = stock {
                            output_found = true; // Set flag if output exists
                            println!("{}", output);
                        }
                    }
                    if !output_found {
                        println!("No results found for the query.");
                    }
                }
                Err(e) => {
                    eprintln!("Query error: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to prepare query: {}", e);
        }
    }

    Ok(String::from(user_query))
}
