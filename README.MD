# Rustbricks

This Rust client facilitates interaction with Databricks clusters, offering a suite of functions to manage clusters and execute SQL statements seamlessly.

## Features

- **Cluster Management**: Obtain detailed cluster information, including status, configuration, and statistics.
- **SQL Execution**: Execute SQL statements directly on your Databricks clusters and retrieve the results.
- **Robust Error Handling**: Custom error types for clear, concise error management across the client.

## Project Structure

The client is modular, with distinct responsibilities separated into various modules for ease of navigation and maintenance:

- **`models`**: Defines data structures for API requests and responses, including `SqlStatementRequest` and `ClusterInfo`.
- **`services`**: Core functionality for interacting with the Databricks API, such as `get_cluster_info`, `execute_sql_statement`, and `get_sql_statement_status`.
- **`utils`**: Utility functions and structures supporting the broader application logic.
- **`errors`**: Custom error types for nuanced error handling throughout the client's operations.

## Getting Started

### Prerequisites

Ensure you have the following environment variables configured:

- `DATABRICKS_HOST`: Your Databricks instance host URL.
- `DATABRICKS_TOKEN`: Your Databricks API token.

These can be set directly in your shell environment or managed via a `.env` file with the help of the [dotenv](https://github.com/dotenv-rs/dotenv) crate.

### Setup

Clone the repository and navigate into the project directory:

```sh
git clone <repository-url>
cd <project-directory>
```

### Building the Project

Build the project using Cargo, Rust's package manager:

```sh
cargo build
```

This compiles the project and its dependencies.

### Running the Client

To execute the client:

```sh
cargo run
```

### Testing

Run the suite of automated tests to ensure everything is functioning as expected:

```sh
cargo test
```

## Dependencies

This client leverages several external crates to enhance its functionality:

- **`chrono`**: Date and time operations.
- **`reqwest`**: HTTP requests.
- **`serde`**: Data serialization and deserialization.
- **`tokio`**: Asynchronous runtime.
- **`actix-*` crates**: Web application framework.

For a complete list of dependencies, refer to the `Cargo.toml` file.

## Examples

The `examples` directory contains sample scripts demonstrating how to use the client effectively. For example, `execute_sql_statement.rs` illustrates executing a SQL statement on a Databricks cluster.
