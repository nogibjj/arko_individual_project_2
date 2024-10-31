[![Rust CI/CD](https://github.com/nogibjj/arko_individual_project_2/actions/workflows/CICD.yml/badge.svg)](https://github.com/nogibjj/arko_individual_project_2/actions/workflows/CICD.yml)
# Stock Data ETL CLI using Rust

## Overview

This project is a command-line interface (CLI) tool for extracting, transforming, loading, and querying stock data, specifically for Apple Inc. (AAPL). The system allows users to extract stock data from a CSV file, load it into a SQLite database, and perform various SQL queries, including CRUD operations. The tool also provides a default mode that returns the percentage change in the closing price over the past five days.

## Project Structure

```
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── Makefile
├── README.md
├── aapl.db
├── data
│   └── AAPL.csv
├── src
│   ├── extract.rs
│   ├── load_transform.rs
│   ├── main.rs
│   └── query.rs
└── tests
    └── cli_tests.rs
```

### File Descriptions

- **Cargo.toml**: The manifest file for the Rust project, listing dependencies and metadata.
- **Cargo.lock**: Automatically generated file that specifies the exact versions of dependencies used in the project.
- **LICENSE**: The license under which the project is distributed.
- **Makefile**: A file for automating the build process and other tasks.
- **README.md**: This documentation file.
- **aapl.db**: The SQLite database file that stores the loaded stock data.
- **data/AAPL.csv**: The source CSV file containing stock data for Apple Inc.
- **src/extract.rs**: Contains functions to extract data from the CSV file and save it locally.
- **src/load_transform.rs**: Contains functions to load the extracted data into the SQLite database and perform any necessary transformations.
- **src/main.rs**: The entry point of the application, implementing the CLI interface.
- **src/query.rs**: Contains functions to query the SQLite database, including the default and custom modes for SQL commands.
- **tests/cli_tests.rs**: Contains tests for the CLI functionality to ensure it works as intended.

## Getting Started

### Prerequisites

- Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).
- SQLite installed for managing the database.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/nogibjj/arko_individual_project_2.git
   cd arko_individual_project_2
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

### Running the Project

1. **Extracting Data**:
   - Use the `extract` module to extract data from the CSV file:
   
   ```bash
   cargo run --bin rust_etl_cli extract
   ```

2. **Loading Data**:
   - Load the extracted data into the SQLite database using the `load_transform` module:
   
   ```bash
   cargo run --bin rust_etl_cli load
   ```

3. **Running Queries**:
   - Use the `main` module to run the CLI and query the database:
   
   ```bash
   cargo run --bin rust_etl_cli query
   ```

   - **Default Mode**: When you run the query script without any additional parameters, it will return the percentage change in the closing price for the past five days.

   - **Custom Mode**: You can pass any SQL command as an argument, including `UPDATE` and `DELETE` actions. For example:


### Testing

To run the CLI tests, use the following command:

```bash
cargo test
```

## Usage

- After running the main script, you can enter SQL commands directly into the prompt. Ensure that your commands are valid SQL statements compatible with SQLite.
- Use the default mode for quick insights on the stock's performance without needing to input SQL commands manually.

## Contributing

Contributions are welcome! If you have suggestions for improvements or find bugs, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

