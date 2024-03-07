# Rustbricks

Rustbricks offers a Rust-based framework designed for efficient integration with the Databricks REST API, enabling Rust applications to effortlessly interact with Databricks services. This library facilitates executing SQL statements, managing clusters, and simplifying the request-response cycle with Databricks.

## Features

- **SQL Execution**: Execute SQL statements directly from Rust.
- **Cluster Management**: Access and manage Databricks cluster information.
- **Persistent Connections**: Utilize a session-based approach to manage persistent connections for improved performance.

## Installation

To use Rustbricks, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rustbricks = "0.1.1"
```

## Quick Start

The following example demonstrates how to execute a SQL statement using Rustbricks with the new `DatabricksSession`:

```rust
use rustbricks::{
    config::Config,
    models::{SqlStatementRequest, SqlStatementResponse},
    services::DatabricksSession,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the configuration
    let config = Config::new()?;
    
    // Create a new session with the configuration
    let session = DatabricksSession::new(config)?;

    // Define a SQL statement request
    let request_body = SqlStatementRequest {
        statement: "SELECT * FROM range(10)".to_string(),
        warehouse_id: "abcdefg123456789".to_string(),
        catalog: None,
        schema: None,
        parameters: None,
        row_limit: None,
        byte_limit: None,
        disposition: "INLINE".to_string(),
        format: "JSON_ARRAY".to_string(),
        wait_timeout: Some("10s".to_string()),
        on_wait_timeout: Some("CONTINUE".to_string()),
    };

    // Execute the SQL statement using the session
    let response = session.execute_sql_statement(request_body).await?;

    // Print the response
    println!("{:#?}", response);

    Ok(())
}
```

## Documentation

For detailed documentation, including all available functions and their usage, please refer to the Rustbricks documentation on docs.rs.

## License

Rustbricks is available under the MIT license. See the [LICENSE](https://github.com/joezug/rustbricks/blob/main/LICENSE) file for more info.
