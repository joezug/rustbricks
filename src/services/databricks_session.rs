use crate::{
    config::Config,
    errors::{ErrorResponse, HttpError},
    models::{ResultData, SqlStatementRequest, SqlStatementResponse, ClusterInfo},
};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Method, StatusCode
};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub struct DatabricksSession {
    client: Arc<Client>,
    config: Config,
}

impl DatabricksSession {
    pub fn new(config: Config) -> Result<Self, reqwest::Error> {
        Self::with_active_pools(12, config)
    }

    pub fn with_active_pools(pool_max_idle_per_host: usize, config: Config) -> Result<Self, reqwest::Error> {
        let client: Client = Client::builder()
            .pool_max_idle_per_host(pool_max_idle_per_host)
            .build()?;

        Ok(DatabricksSession {
            client: Arc::new(client),
            config,
        })
    }

    pub async fn execute_sql_statement(
        &self,
        request_body: SqlStatementRequest,
    ) -> Result<SqlStatementResponse, HttpError> {
        self.send_databricks_request(Method::POST, "api/2.0/sql/statements", Some(request_body))
            .await
    }

    pub async fn get_sql_statement_status(
        &self,
        statement_id: &str,
    ) -> Result<SqlStatementResponse, HttpError> {
        self.send_databricks_request(
            Method::GET,
            &format!("api/2.0/sql/statements/{}", statement_id),
            None::<()>,
        )
        .await
    }

    pub async fn get_sql_statement_result_chunk(
        &self,
        statement_id: &str,
        chunk_index: i32,
    ) -> Result<ResultData, HttpError> {
        self.send_databricks_request(
            Method::GET,
            &format!(
                "api/2.0/sql/statements/{}/result/chunks/{}",
                statement_id, chunk_index
            ),
            None::<()>,
        )
        .await
    }

    pub async fn get_cluster_info(
        &self,
        cluster_id: &str,
    ) -> Result<ClusterInfo, HttpError> {
        self.send_databricks_request(
            Method::GET,
            &format!("api/2.0/clusters/get?cluster_id={}", cluster_id),
            None::<()>, // No body for GET request
        )
        .await
    }

    async fn send_databricks_request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        body: Option<B>,
    ) -> Result<T, HttpError> {
        let url: String = format!("{}/{}", self.config.databricks_host, endpoint);

        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.config.databricks_token)
                .parse()
                .unwrap(),
        );

        let request_builder: reqwest::RequestBuilder =
            self.client.request(method, &url).headers(headers);

        let request_builder: reqwest::RequestBuilder = if let Some(body) = body {
            request_builder.json(&body)
        } else {
            request_builder
        };

        let response = request_builder.send().await.map_err(|err| {
            if err.is_timeout() {
                HttpError::TemporarilyUnavailable(err.to_string())
            } else {
                HttpError::InternalServerError(err.to_string())
            }
        })?;

        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, HttpError> {
        let status: StatusCode = response.status();
        let body_text: String = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to get response text".to_string());

        match status {
            reqwest::StatusCode::OK => serde_json::from_str::<T>(&body_text)
                .map_err(|err| HttpError::InternalServerError(err.to_string())),
            _ => {
                let error: ErrorResponse =
                    serde_json::from_str(&body_text).unwrap_or(ErrorResponse {
                        error_code: "UNKNOWN".to_string(),
                        message: format!("Unknown error with status code: {}", status),
                    });
                Err(HttpError::from_error_response(error))
            }
        }
    }
}
