extern crate rustbricks;

use rustbricks::config::Config;
use rustbricks::models::{SqlStatementRequest, SqlStatementResponse};
use rustbricks::services::execute_sql_statement;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::new()?;
    let warehouse_id_sample: &str = "abcdefg123456789";

    let request_body: SqlStatementRequest = SqlStatementRequest {
        statement: "SELECT * FROM range(10)".to_string(),
        warehouse_id: warehouse_id_sample.to_string(),
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

    let response: SqlStatementResponse = execute_sql_statement(
        &config.databricks_host,
        &config.databricks_token,
        request_body,
    )
    .await
    .unwrap();

    println!("{:#?}", response);

    Ok(())
}
