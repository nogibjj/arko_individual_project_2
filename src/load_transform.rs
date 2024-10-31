use rusqlite::{params, Connection, Result};

pub fn load_data() -> Result<String, std::io::Error> {
    let conn = Connection::open("aapl.db").expect("Failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS AAPL (
            date TEXT PRIMARY KEY,
            open REAL,
            high REAL,
            low REAL,
            close REAL,
            volume INTEGER
        )",
        [],
    )
    .expect("Failed to create table");

    let mut rdr = csv::Reader::from_path("data/AAPL.csv").expect("Failed to read CSV file");

    for result in rdr.records() {
        let record = result.expect("Failed to read record");
        let date = record[0].to_string();
        let open: f64 = record[1].parse().expect("Failed to parse open price");
        let high: f64 = record[2].parse().expect("Failed to parse high price");
        let low: f64 = record[3].parse().expect("Failed to parse low price");
        let close: f64 = record[4].parse().expect("Failed to parse close price");
        let volume: i64 = record[5].parse().expect("Failed to parse volume");

        conn.execute(
            "INSERT OR REPLACE INTO AAPL (date, open, high, low, close, volume) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![date, open, high, low, close, volume],
        ).expect("Failed to insert data into database");
    }

    println!("Data loaded into database");
    Ok(String::from("Loaded"))
}
