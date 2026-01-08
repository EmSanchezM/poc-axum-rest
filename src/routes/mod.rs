use axum::{routing::get, Router};

use crate::{database::DatabasePool, handlers};

pub fn create_routes() -> Router<DatabasePool> {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/error", get(handlers::example_error))
}