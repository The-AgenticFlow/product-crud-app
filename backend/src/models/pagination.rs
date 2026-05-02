//! Pagination response structures
//!
//! This module contains structures for paginated API responses

use serde::{Deserialize, Serialize};

/// Pagination metadata for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page number (1-indexed)
    pub page: u32,

    /// Number of items per page
    pub limit: u32,

    /// Total number of items matching the query
    pub total: i64,

    /// Total number of pages
    pub total_pages: u32,
}

impl Pagination {
    /// Create new pagination metadata
    ///
    /// # Arguments
    /// * `page` - Current page number (1-indexed)
    /// * `limit` - Number of items per page
    /// * `total` - Total number of items
    pub fn new(page: u32, limit: u32, total: i64) -> Self {
        let total_pages = if limit > 0 {
            ((total as f64) / (limit as f64)).ceil() as u32
        } else {
            0
        };

        Self {
            page,
            limit,
            total,
            total_pages,
        }
    }
}

/// Generic paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The data items for the current page
    pub data: Vec<T>,

    /// Pagination metadata
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    ///
    /// # Arguments
    /// * `data` - Items for the current page
    /// * `page` - Current page number
    /// * `limit` - Number of items per page
    /// * `total` - Total number of items
    pub fn new(data: Vec<T>, page: u32, limit: u32, total: i64) -> Self {
        Self {
            data,
            pagination: Pagination::new(page, limit, total),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_creation() {
        let pagination = Pagination::new(1, 20, 100);

        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.limit, 20);
        assert_eq!(pagination.total, 100);
        assert_eq!(pagination.total_pages, 5);
    }

    #[test]
    fn test_pagination_rounding_up() {
        // 101 items with 20 per page = 6 pages (5 full pages + 1 item)
        let pagination = Pagination::new(1, 20, 101);

        assert_eq!(pagination.total_pages, 6);
    }

    #[test]
    fn test_pagination_zero_total() {
        let pagination = Pagination::new(1, 20, 0);

        assert_eq!(pagination.total_pages, 0);
    }

    #[test]
    fn test_paginated_response() {
        let data = vec!["item1", "item2", "item3"];
        let response = PaginatedResponse::new(data.clone(), 1, 20, 100);

        assert_eq!(response.data, data);
        assert_eq!(response.pagination.page, 1);
        assert_eq!(response.pagination.limit, 20);
        assert_eq!(response.pagination.total, 100);
        assert_eq!(response.pagination.total_pages, 5);
    }

    #[test]
    fn test_pagination_serialization() {
        let pagination = Pagination::new(2, 10, 25);
        let json = serde_json::to_string(&pagination).unwrap();

        assert!(json.contains("\"page\":2"));
        assert!(json.contains("\"limit\":10"));
        assert!(json.contains("\"total\":25"));
        assert!(json.contains("\"total_pages\":3"));
    }

    #[test]
    fn test_paginated_response_serialization() {
        let data = vec![1, 2, 3];
        let response = PaginatedResponse::new(data, 1, 10, 30);
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"data\":[1,2,3]"));
        assert!(json.contains("\"pagination\":{"));
    }
}
