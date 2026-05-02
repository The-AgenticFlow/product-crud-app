//! Product handlers for the API
//!
//! This module contains HTTP request handlers for product endpoints

use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::db::{list_products, ProductRepositoryError};
use crate::models::{ListProductsQuery, PaginatedResponse, Product};

/// Error response structure
#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// List products handler
///
/// # Arguments
/// * `Query(query)` - Query parameters for filtering, sorting, and pagination
/// * `Extension(pool)` - Database connection pool
///
/// # Returns
/// A JSON response with paginated products or an error
pub async fn list_products_handler(
    Query(query): Query<ListProductsQuery>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<PaginatedResponse<Product>>, (StatusCode, Json<ErrorResponse>)> {
    // Call the repository function to get products
    let (products, total) = list_products(&pool, &query)
        .await
        .map_err(|e| match e {
            ProductRepositoryError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: msg }))
            }
            ProductRepositoryError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".to_string(),
                    }),
                )
            }
        })?;

    // Build the paginated response
    let response = PaginatedResponse::new(products, query.page, query.limit, total);

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_serialization() {
        let error = ErrorResponse {
            error: "Invalid parameter".to_string(),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("\"error\":\"Invalid parameter\""));
    }
}
