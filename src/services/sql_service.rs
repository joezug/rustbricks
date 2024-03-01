use crate::errors::HttpError;
use crate::models::{ResultData, SqlStatementRequest, SqlStatementResponse};
use crate::services::send_databricks_request;
use reqwest::Method;

pub async fn execute_sql_statement(
    host: &str,
    token: &str,
    request_body: SqlStatementRequest,
) -> Result<SqlStatementResponse, HttpError> {
    send_databricks_request(
        host,
        token,
        Method::POST,
        "api/2.0/sql/statements",
        Some(request_body),
    )
    .await
}

pub async fn get_sql_statement_status(
    host: &str,
    token: &str,
    statement_id: &str,
) -> Result<SqlStatementResponse, HttpError> {
    send_databricks_request(
        host,
        token,
        Method::GET,
        &format!("api/2.0/sql/statements/{}", statement_id),
        None::<()>,
    )
    .await
}

pub async fn get_sql_statement_result_chunk(
    host: &str,
    token: &str,
    statement_id: &str,
    chunk_index: i32,
) -> Result<ResultData, HttpError> {
    send_databricks_request(
        host,
        token,
        Method::GET,
        &format!(
            "api/2.0/sql/statements/{}/result/chunks/{}",
            statement_id, chunk_index
        ),
        None::<()>,
    )
    .await
}
