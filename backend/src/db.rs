//! Database connection pool and migrations
//!
//! This module handles database connection management and migration execution

use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Error};
use std::path::Path;

/// Database connection pool type alias
pub type DbPool = PgPool;

/// Initialize database connection pool
///
/// # Arguments
/// * `database_url` - PostgreSQL connection string
///
/// # Returns
/// A connection pool on success, or an error
pub async fn init_db_pool(database_url: &str) -> Result<DbPool, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

/// Run database migrations
///
/// # Arguments
/// * `pool` - Database connection pool
///
/// # Returns
/// Ok(()) on success, or an error
pub async fn run_migrations(pool: &DbPool) -> Result<(), Error> {
    // Migrations are located in the backend/migrations directory
    let migrator = Migrator::new(Path::new("backend/migrations"))
        .await
        .map_err(|e| Error::Configuration(e.into()))?;

    migrator.run(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // Note: Integration tests that require a running database
    // should be in a separate test file with #[ignore] attribute
    // or use sqlx::testing features
}
