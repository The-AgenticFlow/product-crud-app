//! API routes

pub mod health;

use axum::Router;
use health::health_routes;

/// Creates the main API router with all routes nested under /api
pub fn api_routes() -> Router {
    Router::new().nest("/api", health_routes())
}
