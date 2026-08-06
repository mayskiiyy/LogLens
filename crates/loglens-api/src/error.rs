use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use crate::dto::ProblemDetails;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, title, detail) = match &self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "Bad Request", msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "Unauthorized", msg.clone()),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, "Forbidden", msg.clone()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "Not Found", msg.clone()),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", "An unexpected error occurred".to_string()),
        };

        let body = ProblemDetails {
            r#type: format!("https://loglens.dev/problems/{}", status.as_u16()),
            title: title.to_string(),
            status: status.as_u16(),
            detail,
            instance: "".to_string(),
            request_id: uuid::Uuid::new_v4().to_string(),
        };

        (status, Json(body)).into_response()
    }
}
