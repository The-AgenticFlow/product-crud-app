//! Query parameter structures for API endpoints
//!
//! This module contains structs for parsing and validating query parameters

use serde::{Deserialize, Serialize};

/// Query parameters for listing products
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListProductsQuery {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: u32,

    /// Number of items per page
    #[serde(default = "default_limit")]
    pub limit: u32,

    /// Filter by category (optional)
    pub category: Option<String>,

    /// Search term for name/description (optional)
    pub search: Option<String>,

    /// Field to sort by
    #[serde(default = "default_sort_by")]
    pub sort_by: String,

    /// Sort direction (asc/desc)
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

fn default_sort_by() -> String {
    "created_at".to_string()
}

fn default_sort_order() -> String {
    "desc".to_string()
}

impl ListProductsQuery {
    /// Validate query parameters
    ///
    /// # Errors
    ///
    /// Returns an error message string if validation fails
    pub fn validate(&self) -> Result<(), String> {
        // Validate page
        if self.page < 1 {
            return Err("page must be at least 1".to_string());
        }

        // Validate limit
        if self.limit < 1 || self.limit > 100 {
            return Err("limit must be between 1 and 100".to_string());
        }

        // Validate sort_by (whitelist to prevent SQL injection)
        let allowed_sort_fields = [
            "id",
            "name",
            "price",
            "stock",
            "category",
            "created_at",
            "updated_at",
        ];
        if !allowed_sort_fields.contains(&self.sort_by.as_str()) {
            return Err(format!(
                "sort_by must be one of: {}",
                allowed_sort_fields.join(", ")
            ));
        }

        // Validate sort_order
        if !["asc", "desc"].contains(&self.sort_order.as_str()) {
            return Err("sort_order must be 'asc' or 'desc'".to_string());
        }

        Ok(())
    }

    /// Calculate OFFSET for SQL query
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let query = ListProductsQuery {
            page: default_page(),
            limit: default_limit(),
            category: None,
            search: None,
            sort_by: default_sort_by(),
            sort_order: default_sort_order(),
        };

        assert_eq!(query.page, 1);
        assert_eq!(query.limit, 20);
        assert_eq!(query.sort_by, "created_at");
        assert_eq!(query.sort_order, "desc");
    }

    #[test]
    fn test_validation_valid_query() {
        let query = ListProductsQuery {
            page: 1,
            limit: 20,
            category: Some("Electronics".to_string()),
            search: Some("laptop".to_string()),
            sort_by: "name".to_string(),
            sort_order: "asc".to_string(),
        };

        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_page() {
        let query = ListProductsQuery {
            page: 0,
            limit: 20,
            category: None,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: "desc".to_string(),
        };

        assert!(query.validate().is_err());
        assert!(query.validate().unwrap_err().contains("page must be at least 1"));
    }

    #[test]
    fn test_validation_invalid_limit() {
        let query = ListProductsQuery {
            page: 1,
            limit: 150,
            category: None,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: "desc".to_string(),
        };

        assert!(query.validate().is_err());
        assert!(query.validate().unwrap_err().contains("limit must be between 1 and 100"));
    }

    #[test]
    fn test_validation_invalid_sort_by() {
        let query = ListProductsQuery {
            page: 1,
            limit: 20,
            category: None,
            search: None,
            sort_by: "invalid_field".to_string(),
            sort_order: "desc".to_string(),
        };

        assert!(query.validate().is_err());
        assert!(query.validate().unwrap_err().contains("sort_by must be one of"));
    }

    #[test]
    fn test_validation_invalid_sort_order() {
        let query = ListProductsQuery {
            page: 1,
            limit: 20,
            category: None,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: "invalid".to_string(),
        };

        assert!(query.validate().is_err());
        assert!(query.validate().unwrap_err().contains("sort_order must be 'asc' or 'desc'"));
    }

    #[test]
    fn test_offset_calculation() {
        let query = ListProductsQuery {
            page: 3,
            limit: 20,
            category: None,
            search: None,
            sort_by: "created_at".to_string(),
            sort_order: "desc".to_string(),
        };

        assert_eq!(query.offset(), 40); // (3-1) * 20 = 40
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "page": 2,
            "limit": 50,
            "category": "Books",
            "search": "rust",
            "sort_by": "price",
            "sort_order": "asc"
        }"#;

        let query: ListProductsQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.page, 2);
        assert_eq!(query.limit, 50);
        assert_eq!(query.category, Some("Books".to_string()));
        assert_eq!(query.search, Some("rust".to_string()));
        assert_eq!(query.sort_by, "price");
        assert_eq!(query.sort_order, "asc");
    }

    #[test]
    fn test_deserialization_with_defaults() {
        let json = r#"{}"#;

        let query: ListProductsQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.page, 1);
        assert_eq!(query.limit, 20);
        assert_eq!(query.sort_by, "created_at");
        assert_eq!(query.sort_order, "desc");
    }
}
