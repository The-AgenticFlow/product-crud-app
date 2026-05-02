//! Product handlers for the API
//!
//! This module contains HTTP request handlers for product endpoints

use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{get_product_by_id, list_products, update_product, ProductRepositoryError};
use crate::error::AppError;
use crate::models::{ListProductsQuery, PaginatedResponse, Product, UpdateProduct, UpdateProductRequest};

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
) -> Result<Json<PaginatedResponse<Product>>, AppError> {
    // Call the repository function to get products
    let (products, total) = list_products(&pool, &query).await.map_err(|e| match e {
        ProductRepositoryError::NotFound => {
            // This shouldn't happen in list_products, but we handle it anyway
            AppError::not_found("Product", "unknown")
        }
        ProductRepositoryError::ValidationError(msg) => {
            AppError::validation_error("query", msg)
        }
        ProductRepositoryError::DatabaseError(msg) => {
            AppError::database("Failed to retrieve products", msg)
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
) -> Result<Json<DataResponse<Product>>, AppError> {
    // Parse the UUID from the path parameter
    let id = Uuid::parse_str(&id_str)?;

    // Fetch the product from the database
    let product = get_product_by_id(&pool, id)
        .await
        .map_err(|e| AppError::database("Failed to retrieve product", e.to_string()))?;

    // Return 404 if product not found
    let product = product.ok_or_else(|| AppError::not_found("Product", id.to_string()))?;

    // Return the product wrapped in {"data": {...}}
    Ok(Json(DataResponse { data: product }))
}

/// Update an existing product
///
/// # Arguments
/// * `Path(id_str)` - Path parameter containing the product ID (as string)
/// * `Json(request)` - JSON body with update request
/// * `Extension(pool)` - Database connection pool
///
/// # Returns
/// A JSON response with the updated product wrapped in {"data": {...}} or an error
pub async fn update_product_handler(
    Path(id_str): Path<String>,
    Extension(pool): Extension<PgPool>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<Json<DataResponse<Product>>, AppError> {
    // Parse the UUID from the path parameter
    let id = Uuid::parse_str(&id_str)?;

    // Validate the request
    request.validate().map_err(AppError::validation)?;

    // Convert request to UpdateProduct
    let update: UpdateProduct = request.into();

    // Update the product in the database
    let product = update_product(&pool, id, update).await.map_err(|e| match e {
        ProductRepositoryError::NotFound => AppError::not_found("Product", id.to_string()),
        ProductRepositoryError::DatabaseError(msg) => {
            AppError::database("Failed to update product", msg)
        }
        ProductRepositoryError::ValidationError(msg) => {
            AppError::validation_error("update", msg)
        }
    })?;

    // Return the updated product wrapped in {"data": {...}}
    Ok(Json(DataResponse { data: product }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_response_serialization() {
        let product = Product {
            id: Uuid::nil(),
            name: "Test Product".to_string(),
            description: Some("A test product".to_string()),
            price: rust_decimal::Decimal::new(100, 0),
            stock: 10,
            category: Some("test".to_string()),
            image_url: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let response = DataResponse { data: product.clone() };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"data\""));
        assert!(json.contains("\"Test Product\""));
    }
}
