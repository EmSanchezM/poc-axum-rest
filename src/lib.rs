pub mod config;
pub mod database;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;

use axum::Router;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

pub async fn run() -> anyhow::Result<()> {
    let config = config::AppConfig::from_env()?;
    
    let pool = database::create_pool(&config.database_url).await?;
    
    let app = Router::new()
        .merge(routes::create_routes())
        .layer(axum::middleware::from_fn(crate::middleware::error_handler))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    
    tracing::info!("Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}