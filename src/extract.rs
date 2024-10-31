use reqwest::blocking::get; // Importing the reqwest blocking get function
use std::fs::{self, File}; // Importing fs and File for file operations
use std::io::{self, Write}; // Importing io for handling input/output
use std::path::Path; // Importing Path for handling file paths

pub fn extract_data() -> io::Result<String> {
    // Check if url and file_path are provided
    let url = "https://raw.githubusercontent.com/MainakRepositor/Datasets/refs/heads/master/Stocks/AAPL.csv";
    let file_path = "data/AAPL.csv";

    // Create the parent directory if it doesn't exist
    let path = Path::new(file_path);
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    // Fetch data from the URL
    let response = get(url).expect("Failed to fetch data");

    // Write response to the specified file path
    let mut file = File::create(file_path)?;
    file.write_all(response.bytes().expect("Failed to read bytes").as_ref())?;

    Ok(String::from(file_path)) // Return the file path as a String
}
