//! Product routes
//!
//! This module contains routing configuration for product endpoints

use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers::products::{get_product_handler, list_products_handler};

/// Creates the product router with database pool
///
/// # Arguments
/// * `pool` - Database connection pool to be passed to handlers
///
/// # Returns
/// A Router with product endpoints configured
pub fn products_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/products", get(list_products_handler))
        .route("/products/:id", get(get_product_handler))
        .layer(axum::Extension(pool))
}
