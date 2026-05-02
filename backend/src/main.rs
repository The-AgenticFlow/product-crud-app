//! CRUD Application Backend
//!
//! A RESTful API built with Axum framework

mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;

use axum::Router;
use db::DbPool;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use routes::api_routes;

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

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment");

    // Initialize database connection pool
    tracing::info!("Connecting to database...");
    let db_pool = db::init_db_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    // Run database migrations
    tracing::info!("Running database migrations...");
    db::run_migrations(&db_pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database migrations completed successfully");

    // Build application with API routes and database pool in state
    let app = Router::new()
        .merge(api_routes())
        .with_state(db_pool);

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
