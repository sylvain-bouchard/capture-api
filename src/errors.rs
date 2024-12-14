use axum::{http::StatusCode, response::IntoResponse};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

/// A default error response for most API errors.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ApplicationError {
    pub message: String,
    pub id: Uuid,
    #[serde(skip)]
    pub status: StatusCode,
    /// Optional Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl ApplicationError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            id: Uuid::new_v4(),
            status: StatusCode::BAD_REQUEST,
            details: None,
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;
        response
    }
}
