//! API routes

pub mod health;

use axum::Router;
use health::health_routes;

use crate::db::DbPool;

/// Creates the main API router with all routes nested under /api
pub fn api_routes() -> Router<DbPool> {
    Router::new().nest("/api", health_routes())
}
