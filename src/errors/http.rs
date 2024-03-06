use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}

#[derive(Debug)]
pub enum HttpError {
    BadRequest(String),
    Unauthorized(String),
    PermissionDenied(String),
    NotFound(String),
    RequestLimitExceeded(String),
    InternalServerError(String),
    TemporarilyUnavailable(String),
    InternalError(Box<dyn std::error::Error>),
}

impl HttpError {
    pub fn from_error_response(response: ErrorResponse) -> Self {
        match response.error_code.as_str() {
            "BAD_REQUEST" | "INVALID_PARAMETER_VALUE" => HttpError::BadRequest(response.message),
            "UNAUTHORIZED" => HttpError::Unauthorized(response.message),
            "PERMISSION_DENIED" => HttpError::PermissionDenied(response.message),
            "NOT_FOUND" => HttpError::NotFound(response.message),
            "REQUEST_LIMIT_EXCEEDED" => HttpError::RequestLimitExceeded(response.message),
            "INTERNAL_SERVER_ERROR" => HttpError::InternalServerError(response.message),
            "TEMPORARILY_UNAVAILABLE" => HttpError::TemporarilyUnavailable(response.message),
            _ => HttpError::InternalServerError(response.message),
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpError::BadRequest(message)
            | HttpError::Unauthorized(message)
            | HttpError::PermissionDenied(message)
            | HttpError::NotFound(message)
            | HttpError::RequestLimitExceeded(message)
            | HttpError::InternalServerError(message)
            | HttpError::TemporarilyUnavailable(message) => write!(f, "{}", message),
            HttpError::InternalError(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            // Handle other variants accordingly...
            HttpError::InternalError(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
