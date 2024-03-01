# Rustbricks

Rustbricks provides a Rust-based interface for seamless integration with the Databricks REST API, enabling Rust applications to interact with Databricks services efficiently. It simplifies executing SQL statements, managing clusters, and processing Databricks' responses.

## Features

- **SQL Execution**: Directly execute SQL statements from Rust.
- **Cluster Management**: Retrieve and manage Databricks cluster information.
- **Simplified Requests**: Streamline the creation and handling of Databricks requests and responses.

## Installation

To use Rustbricks, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rustbricks = "0.0.1"
```

## Quick Start

Here's a quick example to execute a SQL statement using Rustbricks:

```rust
use rustbricks::config::Config;
use rustbricks::models::SqlStatementRequest;
use rustbricks::services::execute_sql_statement;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;

    let request_body = SqlStatementRequest {
        statement: "SELECT * FROM range(10)".to_string(),
        warehouse_id: "b57b0114ac8d68c4".to_string(),
        // Optional configurations can be omitted for brevity
        catalog: None, schema: None, parameters: None,
        row_limit: None, byte_limit: None,
        disposition: "INLINE".to_string(),
        format: "JSON_ARRAY".to_string(),
        wait_timeout: Some("10s".to_string()),
        on_wait_timeout: Some("CONTINUE".to_string()),
    };

    let response = execute_sql_statement(
        &config.databricks_host,
        &config.databricks_token,
        request_body,
    ).await?;

    println!("{:#?}", response);

    Ok(())
}
```

## Documentation

For detailed documentation, including all available functions and their usage, please refer to the Rustbricks documentation on docs.rs.

## License

Rustbricks is available under the MIT license. See the [LICENSE](https://github.com/joezug/rustbricks/blob/main/LICENSE) file for more info.
