//! CRUD Application Backend
//!
//! A RESTful API built with Axum framework

mod db;
mod handlers;
mod middleware;
mod models;
mod routes;

#[cfg(feature = "database")]
mod config;

use axum::Router;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "database")]
use routes::api_routes_with_pool;

#[cfg(not(feature = "database"))]
use routes::api_routes;

#[cfg(feature = "database")]
use crate::config::DatabaseConfig;

#[cfg(feature = "database")]
use crate::db::{create_pool, run_migrations};

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize database connection pool and build app
    #[cfg(feature = "database")]
    let app = {
        tracing::info!("Initializing database connection pool");

        let db_config = DatabaseConfig::from_env()
            .expect("Failed to load database configuration");

        let pool = create_pool(&db_config)
            .await
            .expect("Failed to create database connection pool");

        tracing::info!("Database connection pool created successfully");

        // Run migrations
        tracing::info!("Running database migrations");
        run_migrations(&pool)
            .await
            .expect("Failed to run database migrations");

        tracing::info!("Database migrations completed successfully");

        Router::new().merge(api_routes_with_pool(pool))
    };

    #[cfg(not(feature = "database"))]
    let app = Router::new().merge(api_routes());

    // Get port from environment or default to 3000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Invalid PORT");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("Starting server on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind");
    axum::serve(listener, app).await.expect("Failed to start server");
}
