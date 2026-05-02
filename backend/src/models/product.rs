//! Product model for database operations

use crate::error::FieldError;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Product entity representing a product in the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub category: Option<String>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Data for creating a new product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub category: Option<String>,
    pub image_url: Option<String>,
}

/// Data for updating an existing product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub category: Option<String>,
    pub image_url: Option<String>,
}

/// Request DTO for creating a new product with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub category: Option<String>,
    pub image_url: Option<String>,
}

impl CreateProductRequest {
    /// Validates the request and returns a list of field errors, if any
    pub fn validate(&self) -> Vec<FieldError> {
        let mut errors = Vec::new();

        // Validate name: required, max 255 chars
        if self.name.trim().is_empty() {
            errors.push(FieldError::new("name", "Name is required"));
        } else if self.name.len() > 255 {
            errors.push(FieldError::new("name", "Name must be 255 characters or less"));
        }

        // Validate description: optional, max 1000 chars
        if let Some(ref desc) = self.description {
            if desc.len() > 1000 {
                errors.push(FieldError::new("description", "Description must be 1000 characters or less"));
            }
        }

        // Validate price: required, must be >= 0
        if self.price < Decimal::ZERO {
            errors.push(FieldError::new("price", "Price must be greater than or equal to 0"));
        }

        // Validate stock: required, must be >= 0
        if self.stock < 0 {
            errors.push(FieldError::new("stock", "Stock must be greater than or equal to 0"));
        }

        // Validate image_url: optional, valid URL format
        if let Some(ref url) = self.image_url {
            if !url.trim().is_empty() {
                // Basic URL validation - check if it starts with http:// or https://
                if !url.starts_with("http://") && !url.starts_with("https://") {
                    errors.push(FieldError::new("image_url", "Image URL must start with http:// or https://"));
                }
            }
        }

        errors
    }

    /// Converts the request into a CreateProduct model
    pub fn into_create_product(self) -> CreateProduct {
        CreateProduct {
            name: self.name,
            description: self.description,
            price: self.price,
            stock: self.stock,
            category: self.category,
            image_url: self.image_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_serialization() {
        let product = Product {
            id: Uuid::nil(),
            name: "Test Product".to_string(),
            description: Some("A test product".to_string()),
            price: Decimal::new(1999, 2), // 19.99
            stock: 100,
            category: Some("Electronics".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&product).expect("Failed to serialize");
        let deserialized: Product =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(product.name, deserialized.name);
        assert_eq!(product.price, deserialized.price);
        assert_eq!(product.stock, deserialized.stock);
    }

    #[test]
    fn test_create_product_deserialization() {
        let json = r#"{
            "name": "New Product",
            "description": "A new product",
            "price": "29.99",
            "stock": 50,
            "category": "Books",
            "image_url": "https://example.com/new.jpg"
        }"#;

        let create_product: CreateProduct =
            serde_json::from_str(json).expect("Failed to deserialize CreateProduct");

        assert_eq!(create_product.name, "New Product");
        assert_eq!(create_product.stock, 50);
    }

    #[test]
    fn test_create_product_request_validation_valid() {
        let request = CreateProductRequest {
            name: "Valid Product".to_string(),
            description: Some("A valid product description".to_string()),
            price: Decimal::new(1999, 2), // 19.99
            stock: 100,
            category: Some("Electronics".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
        };

        let errors = request.validate();
        assert!(errors.is_empty(), "Expected no validation errors");
    }

    #[test]
    fn test_create_product_request_validation_empty_name() {
        let request = CreateProductRequest {
            name: "".to_string(),
            description: None,
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: None,
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "name");
        assert_eq!(errors[0].message, "Name is required");
    }

    #[test]
    fn test_create_product_request_validation_whitespace_name() {
        let request = CreateProductRequest {
            name: "   ".to_string(),
            description: None,
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: None,
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "name");
        assert_eq!(errors[0].message, "Name is required");
    }

    #[test]
    fn test_create_product_request_validation_name_too_long() {
        let request = CreateProductRequest {
            name: "x".repeat(256),
            description: None,
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: None,
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "name");
        assert_eq!(errors[0].message, "Name must be 255 characters or less");
    }

    #[test]
    fn test_create_product_request_validation_description_too_long() {
        let request = CreateProductRequest {
            name: "Product".to_string(),
            description: Some("x".repeat(1001)),
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: None,
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "description");
        assert_eq!(errors[0].message, "Description must be 1000 characters or less");
    }

    #[test]
    fn test_create_product_request_validation_negative_price() {
        let request = CreateProductRequest {
            name: "Product".to_string(),
            description: None,
            price: Decimal::new(-100, 0), // -1.00
            stock: 0,
            category: None,
            image_url: None,
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "price");
        assert_eq!(errors[0].message, "Price must be greater than or equal to 0");
    }

    #[test]
    fn test_create_product_request_validation_negative_stock() {
        let request = CreateProductRequest {
            name: "Product".to_string(),
            description: None,
            price: Decimal::ZERO,
            stock: -1,
            category: None,
            image_url: None,
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "stock");
        assert_eq!(errors[0].message, "Stock must be greater than or equal to 0");
    }

    #[test]
    fn test_create_product_request_validation_invalid_url() {
        let request = CreateProductRequest {
            name: "Product".to_string(),
            description: None,
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: Some("invalid-url".to_string()),
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "image_url");
        assert_eq!(errors[0].message, "Image URL must start with http:// or https://");
    }

    #[test]
    fn test_create_product_request_validation_valid_http_url() {
        let request = CreateProductRequest {
            name: "Product".to_string(),
            description: None,
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: Some("http://example.com/image.jpg".to_string()),
        };

        let errors = request.validate();
        assert!(errors.is_empty(), "Expected no validation errors for http URL");
    }

    #[test]
    fn test_create_product_request_validation_empty_url_allowed() {
        let request = CreateProductRequest {
            name: "Product".to_string(),
            description: None,
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: Some("".to_string()),
        };

        let errors = request.validate();
        assert!(errors.is_empty(), "Empty URL should be allowed");
    }

    #[test]
    fn test_create_product_request_validation_multiple_errors() {
        let request = CreateProductRequest {
            name: "".to_string(),
            description: Some("x".repeat(1001)),
            price: Decimal::new(-100, 0),
            stock: -1,
            category: None,
            image_url: Some("invalid".to_string()),
        };

        let errors = request.validate();
        assert_eq!(errors.len(), 5);
    }

    #[test]
    fn test_create_product_request_into_create_product() {
        let request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: Some("A test product".to_string()),
            price: Decimal::new(1999, 2),
            stock: 100,
            category: Some("Electronics".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
        };

        let create_product = request.into_create_product();
        assert_eq!(create_product.name, "Test Product");
        assert_eq!(create_product.description, Some("A test product".to_string()));
        assert_eq!(create_product.price, Decimal::new(1999, 2));
        assert_eq!(create_product.stock, 100);
        assert_eq!(create_product.category, Some("Electronics".to_string()));
        assert_eq!(create_product.image_url, Some("https://example.com/image.jpg".to_string()));
    }
}
