use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // Build application with a basic route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

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
