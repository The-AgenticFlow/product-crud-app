//! Product repository operations
//!
//! This module contains database operations for products

use sqlx::{PgPool, QueryBuilder};
use crate::models::{ListProductsQuery, Product, UpdateProduct};

/// Error type for product repository operations
#[derive(Debug)]
pub enum ProductRepositoryError {
    /// Database error
    DatabaseError(String),
    /// Validation error
    ValidationError(String),
}

impl std::fmt::Display for ProductRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductRepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ProductRepositoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ProductRepositoryError {}

/// List products with filtering, sorting, and pagination
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `query` - Query parameters for filtering, sorting, and pagination
///
/// # Returns
/// A tuple containing (products, total_count) or an error
///
/// # Errors
/// Returns an error if the query validation fails or database operation fails
#[allow(dead_code)]
pub async fn list_products(
    pool: &PgPool,
    query: &ListProductsQuery,
) -> Result<(Vec<Product>, i64), ProductRepositoryError> {
    // Validate query parameters first
    query.validate().map_err(ProductRepositoryError::ValidationError)?;

    // Build the count query (for total count with filters)
    let total = count_products(pool, query).await?;

    // If total is 0, return early with empty results
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    // Build the query using QueryBuilder
    let mut query_builder = QueryBuilder::new(
        "SELECT id, name, description, price, stock, category, image_url, created_at, updated_at FROM products WHERE 1=1"
    );

    // Add category filter
    if let Some(ref category) = query.category {
        query_builder.push(" AND category = ");
        query_builder.push_bind(category);
    }

    // Add search filter (case-insensitive search in name or description)
    if let Some(ref search) = query.search {
        query_builder.push(" AND (name ILIKE ");
        let search_pattern = format!("%{}%", search);
        query_builder.push_bind(search_pattern);
        query_builder.push(" OR description ILIKE ");
        let search_pattern2 = format!("%{}%", search);
        query_builder.push_bind(search_pattern2);
        query_builder.push(")");
    }

    // Add ORDER BY clause - we use the validated sort_by and sort_order from the query
    // which are already validated against a whitelist to prevent SQL injection
    query_builder.push(format!(" ORDER BY {} {}", query.sort_by, query.sort_order.to_uppercase()).as_str());

    // Add LIMIT and OFFSET
    query_builder.push(" LIMIT ");
    query_builder.push_bind(query.limit as i64);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(query.offset() as i64);

    // Execute the query
    let products = query_builder
        .build_query_as::<Product>()
        .fetch_all(pool)
        .await
        .map_err(|e| ProductRepositoryError::DatabaseError(e.to_string()))?;

    Ok((products, total))
}

/// Count total products matching filters
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `query` - Query parameters for filtering (only category and search are used)
///
/// # Returns
/// The total count of products matching the filters
///
/// # Errors
/// Returns an error if the database operation fails
#[allow(dead_code)]
pub async fn count_products(
    pool: &PgPool,
    query: &ListProductsQuery,
) -> Result<i64, ProductRepositoryError> {
    // Build the count query using QueryBuilder
    let mut query_builder = QueryBuilder::new("SELECT COUNT(*) as count FROM products WHERE 1=1");

    // Add category filter
    if let Some(ref category) = query.category {
        query_builder.push(" AND category = ");
        query_builder.push_bind(category);
    }

    // Add search filter (case-insensitive search in name or description)
    if let Some(ref search) = query.search {
        query_builder.push(" AND (name ILIKE ");
        let search_pattern = format!("%{}%", search);
        query_builder.push_bind(search_pattern);
        query_builder.push(" OR description ILIKE ");
        let search_pattern2 = format!("%{}%", search);
        query_builder.push_bind(search_pattern2);
        query_builder.push(")");
    }

    // Execute the query
    let count: i64 = query_builder
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|e| ProductRepositoryError::DatabaseError(e.to_string()))?;

    Ok(count)
}

/// Get a single product by ID
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `id` - UUID of the product to fetch
///
/// # Returns
/// The product if found, or None if not found
///
/// # Errors
/// Returns an error if the database operation fails
#[allow(dead_code)]
pub async fn get_product_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Option<Product>, ProductRepositoryError> {
    let product = sqlx::query_as::<_, Product>(
        "SELECT id, name, description, price, stock, category, image_url, created_at, updated_at FROM products WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| ProductRepositoryError::DatabaseError(e.to_string()))?;

    Ok(product)
}

/// Delete a product by ID
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `id` - UUID of the product to delete
///
/// # Returns
/// `Ok(true)` if the product was deleted, `Ok(false)` if the product was not found
///
/// # Errors
/// Returns an error if the database operation fails
pub async fn delete_product(pool: &PgPool, id: uuid::Uuid) -> Result<bool, ProductRepositoryError> {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| ProductRepositoryError::DatabaseError(e.to_string()))?;

    Ok(result.rows_affected() > 0)
}

/// Update a product by ID with partial updates
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `id` - UUID of the product to update
/// * `update_data` - UpdateProduct struct containing fields to update
///
/// # Returns
/// `Ok(Some(product))` if the product was updated successfully
/// `Ok(None)` if the product was not found
///
/// # Errors
/// Returns an error if the database operation fails
///
/// # Notes
/// - Only fields with `Some(_)` values are updated
/// - The `updated_at` timestamp is automatically set to NOW()
/// - Fields with `None` values retain their original values
pub async fn update_product(
    pool: &PgPool,
    id: uuid::Uuid,
    update_data: UpdateProduct,
) -> Result<Option<Product>, ProductRepositoryError> {
    // Check if there are any fields to update
    let has_updates = update_data.name.is_some()
        || update_data.description.is_some()
        || update_data.price.is_some()
        || update_data.stock.is_some()
        || update_data.category.is_some()
        || update_data.image_url.is_some();

    // If no fields to update, just update the timestamp and return the product
    if !has_updates {
        let product = sqlx::query_as::<_, Product>(
            "UPDATE products SET updated_at = NOW() WHERE id = $1 \
             RETURNING id, name, description, price, stock, category, image_url, created_at, updated_at"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ProductRepositoryError::DatabaseError(e.to_string()))?;

        return Ok(product);
    }

    // Build dynamic UPDATE query with QueryBuilder
    let mut query_builder = QueryBuilder::new(
        "UPDATE products SET updated_at = NOW()"
    );

    // Add fields to update only if they are Some(_)
    if let Some(ref name) = update_data.name {
        query_builder.push(", name = ");
        query_builder.push_bind(name);
    }

    if let Some(ref description) = update_data.description {
        query_builder.push(", description = ");
        query_builder.push_bind(description);
    }

    if let Some(price) = update_data.price {
        query_builder.push(", price = ");
        query_builder.push_bind(price);
    }

    if let Some(stock) = update_data.stock {
        query_builder.push(", stock = ");
        query_builder.push_bind(stock);
    }

    if let Some(ref category) = update_data.category {
        query_builder.push(", category = ");
        query_builder.push_bind(category);
    }

    if let Some(ref image_url) = update_data.image_url {
        query_builder.push(", image_url = ");
        query_builder.push_bind(image_url);
    }

    // Add WHERE clause and RETURNING
    query_builder.push(" WHERE id = ");
    query_builder.push_bind(id);
    query_builder.push(" RETURNING id, name, description, price, stock, category, image_url, created_at, updated_at");

    // Execute the query
    let product = query_builder
        .build_query_as::<Product>()
        .fetch_optional(pool)
        .await
        .map_err(|e| ProductRepositoryError::DatabaseError(e.to_string()))?;

    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ProductRepositoryError::DatabaseError("connection failed".to_string());
        assert_eq!(format!("{}", error), "Database error: connection failed");

        let error = ProductRepositoryError::ValidationError("invalid page".to_string());
        assert_eq!(format!("{}", error), "Validation error: invalid page");
    }

    #[test]
    fn test_query_validation_in_list_products() {
        // Test that invalid queries are caught
        let invalid_query = ListProductsQuery {
            page: 0,
            limit: 20,
            category: None,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: "desc".to_string(),
        };

        // The validate function should return an error
        assert!(invalid_query.validate().is_err());
    }

    #[test]
    fn test_offset_calculation() {
        let query = ListProductsQuery {
            page: 1,
            limit: 20,
            category: None,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: "desc".to_string(),
        };

        assert_eq!(query.offset(), 0);

        let query = ListProductsQuery {
            page: 3,
            limit: 20,
            category: None,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: "desc".to_string(),
        };

        assert_eq!(query.offset(), 40);
    }

    #[test]
    fn test_update_product_struct_creation() {
        use crate::models::UpdateProduct;
        use rust_decimal::Decimal;

        // Test creating an UpdateProduct with all fields
        let update = UpdateProduct {
            name: Some("Updated Name".to_string()),
            description: Some("Updated description".to_string()),
            price: Some(Decimal::new(1999, 2)),
            stock: Some(50),
            category: Some("Electronics".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
        };

        assert_eq!(update.name, Some("Updated Name".to_string()));
        assert_eq!(update.stock, Some(50));
    }

    #[test]
    fn test_update_product_partial_update() {
        use crate::models::UpdateProduct;
        use rust_decimal::Decimal;

        // Test creating an UpdateProduct with only some fields
        let update = UpdateProduct {
            name: Some("New Name".to_string()),
            description: None,
            price: Some(Decimal::new(999, 2)),
            stock: None,
            category: None,
            image_url: None,
        };

        assert_eq!(update.name, Some("New Name".to_string()));
        assert_eq!(update.price, Some(Decimal::new(999, 2)));
        assert!(update.description.is_none());
        assert!(update.stock.is_none());
    }

    #[test]
    fn test_update_product_empty_update() {
        use crate::models::UpdateProduct;

        // Test creating an UpdateProduct with no fields
        let update = UpdateProduct {
            name: None,
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: None,
        };

        // All fields should be None
        assert!(update.name.is_none());
        assert!(update.description.is_none());
        assert!(update.price.is_none());
        assert!(update.stock.is_none());
        assert!(update.category.is_none());
        assert!(update.image_url.is_none());
    }
}
