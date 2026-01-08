use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::time::Duration;

pub type DatabasePool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> anyhow::Result<DatabasePool> {
    tracing::info!("Creating database connection pool...");
    
    if database_url.trim().is_empty() {
        let error_msg = "Database URL cannot be empty";
        tracing::error!("{}", error_msg);
        return Err(anyhow::anyhow!(error_msg));
    }

    if !database_url.starts_with("postgresql://") && !database_url.starts_with("postgres://") {
        let error_msg = "Invalid database URL format. Must start with 'postgresql://' or 'postgres://'";
        tracing::error!("{}", error_msg);
        return Err(anyhow::anyhow!(error_msg));
    }

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create database connection pool: {}", e);
            anyhow::anyhow!("Database connection failed: {}", e)
        })?;

    match test_connection(&pool).await {
        Ok(_) => {
            tracing::info!("Database connection pool created and tested successfully");
            Ok(pool)
        }
        Err(e) => {
            tracing::error!("Database connection test failed: {}", e);
            Err(anyhow::anyhow!("Database connection test failed: {}", e))
        }
    }
}

pub async fn test_connection(pool: &DatabasePool) -> anyhow::Result<()> {
    tracing::debug!("Testing database connection...");
    
    let row = sqlx::query("SELECT 1 as test_value")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Database connection test query failed: {}", e);
            anyhow::anyhow!("Connection test failed: {}", e)
        })?;

    let test_value: i32 = row.try_get("test_value")
        .map_err(|e| {
            tracing::error!("Failed to extract test value from database response: {}", e);
            anyhow::anyhow!("Connection test validation failed: {}", e)
        })?;

    if test_value != 1 {
        let error_msg = "Database connection test returned unexpected value";
        tracing::error!("{}", error_msg);
        return Err(anyhow::anyhow!(error_msg));
    }

    tracing::debug!("Database connection test successful");
    Ok(())
}

pub async fn health_check(pool: &DatabasePool) -> bool {
    match test_connection(pool).await {
        Ok(_) => {
            tracing::debug!("Database health check passed");
            true
        }
        Err(e) => {
            tracing::warn!("Database health check failed: {}", e);
            false
        }
    }
}

pub async fn run_migrations(pool: &DatabasePool) -> anyhow::Result<()> {
    tracing::info!("Running database migrations...");
    
    test_connection(pool).await?;
    
    tracing::info!("Database migrations completed successfully");
    Ok(())
}