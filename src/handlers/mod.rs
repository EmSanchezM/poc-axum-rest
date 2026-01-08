use axum::{extract::State, Json};
use serde::Serialize;

use crate::{database::DatabasePool, errors::{AppError, AppResult}};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub database: String,
}

pub async fn health_check(
    State(pool): State<DatabasePool>,
) -> AppResult<Json<HealthResponse>> {

    let database_status = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => "connected".to_string(),
        Err(err) => {
            tracing::warn!("Database health check failed: {}", err);
            "disconnected".to_string()
        }
    };

    let response = HealthResponse {
        status: "ok".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        database: database_status,
    };

    Ok(Json(response))
}

pub async fn example_error() -> AppResult<Json<String>> {
    Err(AppError::ValidationError("This is an example validation error".to_string()))
}