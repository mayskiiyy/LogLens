use axum::extract::State;
use axum::Json;
use loglens_storage::Database;
use crate::dto::HealthResponse;
use crate::error::ApiError;

pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn ready_handler(State(db): State<Database>) -> Result<Json<HealthResponse>, ApiError> {
    sqlx::query("SELECT 1")
        .execute(db.pool())
        .await
        .map_err(|e| ApiError::Internal(format!("Database health check failed: {}", e)))?;

    Ok(Json(HealthResponse {
        status: "ready".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}
