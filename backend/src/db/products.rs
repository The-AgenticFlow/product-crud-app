//! Product repository operations
//!
//! This module contains database operations for products

use sqlx::{PgPool, QueryBuilder};
use crate::models::{ListProductsQuery, Product};

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
}
