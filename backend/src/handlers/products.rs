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

use crate::db::{create_product, get_product_by_id, list_products, ProductRepositoryError};
use crate::error::AppError;
use crate::models::{CreateProductRequest, ListProductsQuery, PaginatedResponse, Product};

/// Single item response wrapper
#[derive(Debug, Serialize)]
pub struct DataResponse<T> {
    pub data: T,
}

/// Response wrapper for created resources with a message
#[derive(Debug, Serialize)]
pub struct CreatedResponse<T> {
    pub data: T,
    pub message: String,
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

/// Create a new product
///
/// # Arguments
/// * `Json(request)` - JSON body with product data
/// * `Extension(pool)` - Database connection pool
///
/// # Returns
/// A 201 Created response with the created product wrapped in {"data": {...}, "message": "..."}
pub async fn create_product_handler(
    Json(request): Json<CreateProductRequest>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<CreatedResponse<Product>>), AppError> {
    // Validate the request
    let errors = request.validate();
    if !errors.is_empty() {
        return Err(AppError::validation(errors));
    }

    // Convert request to CreateProduct model
    let product_data = request.into_create_product();

    // Insert the product into the database
    let product = create_product(&pool, product_data)
        .await
        .map_err(|e| AppError::database("Failed to create product", e.to_string()))?;

    // Return 201 Created with the product and success message
    Ok((
        StatusCode::CREATED,
        Json(CreatedResponse {
            data: product,
            message: "Product created successfully".to_string(),
        }),
    ))
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

    #[test]
    fn test_created_response_serialization() {
        let product = Product {
            id: Uuid::nil(),
            name: "New Product".to_string(),
            description: Some("A newly created product".to_string()),
            price: rust_decimal::Decimal::new(1999, 2),
            stock: 50,
            category: Some("Electronics".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let response = CreatedResponse {
            data: product,
            message: "Product created successfully".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"data\""));
        assert!(json.contains("\"message\""));
        assert!(json.contains("\"Product created successfully\""));
        assert!(json.contains("\"New Product\""));
    }
}
