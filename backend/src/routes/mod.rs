//! API routes

pub mod health;

#[cfg(feature = "database")]
pub mod products;

use axum::Router;
use health::health_routes;

#[cfg(feature = "database")]
use products::products_routes;

#[cfg(feature = "database")]
use sqlx::PgPool;

/// Creates the main API router with all routes nested under /api
#[cfg(not(feature = "database"))]
pub fn api_routes() -> Router {
    Router::new().nest("/api", health_routes())
}

/// Creates the main API router with database pool for routes that need it
#[cfg(feature = "database")]
pub fn api_routes_with_pool(pool: PgPool) -> Router {
    Router::new()
        .nest("/api", health_routes())
        .merge(products_routes(pool))
}
