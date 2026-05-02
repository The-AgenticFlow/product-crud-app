//! Product handlers for the API
//!
//! This module contains HTTP request handlers for product endpoints

use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{get_product_by_id, list_products, ProductRepositoryError};
use crate::models::{ListProductsQuery, PaginatedResponse, Product};

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Single item response wrapper
#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    pub data: T,
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

/// Get a single product by ID
///
/// # Arguments
/// * `Path(id_str)` - Path parameter containing the product ID (as string)
/// * `Extension(pool)` - Database connection pool
///
/// # Returns
/// A JSON response with the product wrapped in {"data": {...}} or an error
pub async fn get_product_handler(
    Path(id_str): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<DataResponse<Product>>, (StatusCode, Json<ErrorResponse>)> {
    // Parse the UUID from the path parameter
    let id = Uuid::parse_str(&id_str).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid UUID format".to_string(),
            }),
        )
    })?;

    // Fetch the product from the database
    let product = get_product_by_id(&pool, id)
        .await
        .map_err(|e| {
            tracing::error!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
        })?;

    // Return 404 if product not found
    let product = product.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Product not found".to_string(),
            }),
        )
    })?;

    // Return the product wrapped in {"data": {...}}
    Ok(Json(DataResponse { data: product }))
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
