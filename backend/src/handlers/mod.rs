//! Request handlers for the API

#[cfg(feature = "database")]
pub mod products;

use axum::Json;
use serde_json::{json, Value};

/// Health check handler
/// Returns a 200 OK with JSON body {"status": "healthy"}
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy"
    }))
}
