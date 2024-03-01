use crate::errors::{ErrorResponse, HttpError};
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Method, RequestBuilder, Response, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};

pub async fn send_databricks_request<T: DeserializeOwned, B: Serialize>(
    host: &str,
    token: &str,
    method: Method,
    endpoint: &str,
    body: Option<B>,
) -> Result<T, HttpError> {
    let client: Client = Client::new();
    let url: String = format!("{}/{}", host, endpoint);

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

    let request_builder: RequestBuilder = client.request(method, url).headers(headers);

    let request_builder: RequestBuilder = if let Some(body) = body {
        request_builder.json(&body)
    } else {
        request_builder
    };

    let response: Response = request_builder.send().await.map_err(|err| {
        if err.is_timeout() {
            HttpError::TemporarilyUnavailable(err.to_string())
        } else {
            HttpError::InternalServerError(err.to_string())
        }
    })?;

    let status: StatusCode = response.status();
    let body_text: String = response
        .text()
        .await
        .unwrap_or_else(|_| "Failed to get response text".to_string());

    match status {
        StatusCode::OK => serde_json::from_str::<T>(&body_text)
            .map_err(|err| HttpError::InternalServerError(err.to_string())),
        _ => {
            let error: ErrorResponse = serde_json::from_str(&body_text).unwrap_or(ErrorResponse {
                error_code: "UNKNOWN".to_string(),
                message: format!("Unknown error with status code: {}", status),
            });
            Err(HttpError::from_error_response(error))
        }
    }
}
