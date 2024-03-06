extern crate rustbricks;

use rustbricks::{
    config::Config,
    models::{SqlStatementRequest, SqlStatementResponse},
    services::DatabricksSession,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::new()?;
    let session: DatabricksSession = DatabricksSession::new(config)?;

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

    let response: SqlStatementResponse = session.execute_sql_statement(request_body).await?;

    println!("{:#?}", response);

    Ok(())
}
