//! Health check routes

use axum::{routing::get, Router};

use crate::db::DbPool;
use crate::handlers::health_check;

/// Creates the health check router
pub fn health_routes() -> Router<DbPool> {
    Router::new().route("/health", get(health_check))
}
