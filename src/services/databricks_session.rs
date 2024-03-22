use crate::{
    config::Config,
    errors::{ErrorResponse, HttpError},
    models::{
        ClusterInfo, JobRunRequest, JobRunResponse, ResultData, SqlStatementRequest,
        SqlStatementResponse,
    },
};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Method, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

pub struct DatabricksSession {
    client: Arc<Client>,
    config: Config,
}

impl DatabricksSession {
    /// Creates a new `DatabricksSession` with the specified configuration.
    ///
    /// This constructor uses the default setting for the maximum number of idle connections
    /// per host (12). It initializes the HTTP client used for communicating with the Databricks API.
    ///
    /// Parameters:
    /// - `config`: A `Config` struct containing the necessary configuration, such as the Databricks
    ///   instance host URL and the authentication token.
    ///
    /// Returns:
    /// - A `Result` containing the new `DatabricksSession` if successful, or a `reqwest::Error` if
    ///   the HTTP client could not be initialized.
    pub fn new(config: Config) -> Result<Self, reqwest::Error> {
        Self::with_active_pools(12, config)
    }

    /// Creates a new `DatabricksSession` with the specified configuration and a custom setting for
    /// the maximum number of idle connections per host.
    ///
    /// This allows more control over the resource utilization of the HTTP client when making
    /// requests to the Databricks API.
    ///
    /// Parameters:
    /// - `pool_max_idle_per_host`: The maximum number of idle connections to maintain per host.
    /// - `config`: A `Config` struct as described in `new`.
    ///
    /// Returns:
    /// - Same as `new`.
    pub fn with_active_pools(
        pool_max_idle_per_host: usize,
        config: Config,
    ) -> Result<Self, reqwest::Error> {
        let client: Client = Client::builder()
            .pool_max_idle_per_host(pool_max_idle_per_host)
            .build()?;

        Ok(DatabricksSession {
            client: Arc::new(client),
            config,
        })
    }

    /// Creates a new `DatabricksSession` that ignores SSL certificate verification errors.
    ///
    /// This is useful for development environments or cases where self-signed certificates are used,
    /// but should be used with caution due to the security implications.
    ///
    /// Parameters:
    /// - `config`: A `Config` struct as described in `new`.
    ///
    /// Returns:
    /// - Same as `new`, but with SSL certificate verification disabled.
    pub fn with_unverified_ssl(config: Config) -> Result<Self, reqwest::Error> {
        let client: Client = Client::builder()
            .pool_max_idle_per_host(12)
            .danger_accept_invalid_certs(true)
            .build()?;

        Ok(DatabricksSession {
            client: Arc::new(client),
            config,
        })
    }

    /// Executes a SQL statement on Databricks and returns the response.
    ///
    /// This method submits a SQL statement for execution and provides the initial response,
    /// which includes details such as the statement ID for subsequent status checks or result retrieval.
    ///
    /// Parameters:
    /// - `request_body`: A `SqlStatementRequest` struct containing the SQL statement to be executed.
    ///
    /// Returns:
    /// - A `Result` containing the `SqlStatementResponse` if successful, or an `HttpError` if the request fails.
    pub async fn execute_sql_statement(
        &self,
        request_body: SqlStatementRequest,
    ) -> Result<SqlStatementResponse, HttpError> {
        self.send_databricks_request(Method::POST, "api/2.0/sql/statements", Some(request_body))
            .await
    }

    /// Retrieves the status of a previously executed SQL statement.
    ///
    /// This method polls the status of a SQL statement execution by its statement ID, allowing clients
    /// to check if the execution has completed and if the results are ready to be fetched.
    ///
    /// Parameters:
    /// - `statement_id`: The ID of the SQL statement execution to check.
    ///
    /// Returns:
    /// - Same as `execute_sql_statement`.
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

    /// Fetches a chunk of the result set from a previously executed SQL statement.
    ///
    /// This method retrieves a specific chunk of the results for a SQL statement execution, identified
    /// by the statement ID and the chunk index.
    ///
    /// Parameters:
    /// - `statement_id`: The ID of the SQL statement execution.
    /// - `chunk_index`: The index of the result chunk to retrieve.
    /// Returns:
    /// - A `Result` containing the `ResultData` for the specified chunk, or an `HttpError` if the request fails.
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

    /// Retrieves information about a specific cluster.
    ///
    /// This method fetches detailed information about a Databricks cluster, identified by the cluster ID.
    ///
    /// Parameters:
    /// - `cluster_id`: The ID of the cluster to retrieve information for.
    ///
    /// Returns:
    /// - A `Result` containing the `ClusterInfo` if successful, or an `HttpError` if the request fails.
    pub async fn get_cluster_info(&self, cluster_id: &str) -> Result<ClusterInfo, HttpError> {
        self.send_databricks_request(
            Method::GET,
            &format!("api/2.0/clusters/get?cluster_id={}", cluster_id),
            None::<()>, // No body for GET request
        )
        .await
    }

    /// A generic method for sending requests to the Databricks API.
    ///
    /// This internal method is a utility function used by other methods to send HTTP requests to the
    /// Databricks API. It handles constructing the request, setting headers, serializing the request body,
    /// and deserializing the response.
    ///
    /// Parameters:
    /// - `method`: The HTTP method to use for the request.
    /// - `endpoint`: The API endpoint to send the request to.
    /// - `body`: An optional request body to serialize and include with the request.
    ///
    /// Returns:
    /// - A `Result` containing the deserialized response body if successful, or an `HttpError` if the request fails.
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

    /// Handles the HTTP response, deserializing the JSON body or converting errors.
    ///
    /// This internal method processes the HTTP response from the Databricks API, attempting to deserialize
    /// the response body into the expected type or converting HTTP errors into `HttpError` instances.
    ///
    /// Parameters:
    /// - `response`: The `reqwest::Response` object to process.
    ///
    /// Returns:
    /// - A `Result` containing the deserialized response body if the request was successful, or an `HttpError`
    ///   if there was an error with the request or response processing.
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

    /// Executes a job run on Databricks using the specified job configuration.
    ///
    /// This asynchronous method sends a request to the Databricks API to trigger
    /// a job run based on the provided job configuration. The method constructs a POST
    /// request to the `/api/2.1/jobs/run-now` endpoint with a `JobRunRequest` body,
    /// which includes various optional parameters to customize the job run.
    ///
    /// Parameters:
    /// - `request_body`: A `JobRunRequest` struct representing the configuration for the job run.
    ///   This includes the job ID, an optional idempotency token, and various parameters
    ///   that can be used to customize the job execution, such as `jar_params`, `notebook_params`,
    ///   `python_params`, and others.
    ///
    /// Returns:
    /// - A `Result<JobRunResponse, HttpError>`: On success, returns a `JobRunResponse` struct
    ///   containing details about the triggered job run, including the `run_id`. On failure,
    ///   returns an `HttpError` indicating what went wrong during the request.
    pub async fn execute_job_run(
        &self,
        request_body: JobRunRequest,
    ) -> Result<JobRunResponse, HttpError> {
        self.send_databricks_request(Method::POST, "api/2.1/jobs/run-now", Some(request_body))
            .await
    }
}
