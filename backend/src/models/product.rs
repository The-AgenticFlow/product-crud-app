//! Product model for database operations

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use url::Url;

use crate::error::FieldError;

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

/// Request payload for creating a new product with validation
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub category: Option<String>,
    pub image_url: Option<String>,
}

impl CreateProductRequest {
    /// Validate the request data
    ///
    /// Returns Ok(()) if validation passes, or Err(Vec<FieldError>) with validation errors
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut errors = Vec::new();

        // Validate name: required, max 255 chars, not empty
        let name_trimmed = self.name.trim();
        if name_trimmed.is_empty() {
            errors.push(FieldError::new("name", "Product name is required"));
        } else if name_trimmed.len() > 255 {
            errors.push(FieldError::new("name", "Product name must not exceed 255 characters"));
        }

        // Validate description: optional, max 1000 chars
        if let Some(ref desc) = self.description {
            if desc.len() > 1000 {
                errors.push(FieldError::new("description", "Description must not exceed 1000 characters"));
            }
        }

        // Validate price: required, must be >= 0
        if self.price < Decimal::ZERO {
            errors.push(FieldError::new("price", "Price must be non-negative"));
        }

        // Validate stock: required, must be >= 0
        if self.stock < 0 {
            errors.push(FieldError::new("stock", "Stock must be non-negative"));
        }

        // Validate image_url: optional, valid URL format
        if let Some(ref url_str) = self.image_url {
            if !url_str.is_empty() {
                // Only validate if URL string is not empty
                if Url::parse(url_str).is_err() {
                    errors.push(FieldError::new("image_url", "Invalid URL format"));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Convert CreateProductRequest to CreateProduct
impl From<CreateProductRequest> for CreateProduct {
    fn from(req: CreateProductRequest) -> Self {
        Self {
            name: req.name,
            description: req.description,
            price: req.price,
            stock: req.stock,
            category: req.category,
            image_url: req.image_url,
        }
    }
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

/// Request payload for updating an existing product with validation
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
    pub category: Option<String>,
    pub image_url: Option<String>,
}

impl UpdateProductRequest {
    /// Validate the request data
    ///
    /// Only validates fields that are provided (Some). Returns Ok(()) if validation passes,
    /// or Err(Vec<FieldError>) with validation errors.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut errors = Vec::new();

        // Validate name if provided: not empty after trim, max 255 chars
        if let Some(ref name) = self.name {
            let name_trimmed = name.trim();
            if name_trimmed.is_empty() {
                errors.push(FieldError::new("name", "Product name cannot be empty"));
            } else if name_trimmed.len() > 255 {
                errors.push(FieldError::new("name", "Product name must not exceed 255 characters"));
            }
        }

        // Validate description if provided: max 1000 chars
        if let Some(ref desc) = self.description {
            if desc.len() > 1000 {
                errors.push(FieldError::new("description", "Description must not exceed 1000 characters"));
            }
        }

        // Validate price if provided: must be >= 0
        if let Some(price) = self.price {
            if price < Decimal::ZERO {
                errors.push(FieldError::new("price", "Price must be non-negative"));
            }
        }

        // Validate stock if provided: must be >= 0
        if let Some(stock) = self.stock {
            if stock < 0 {
                errors.push(FieldError::new("stock", "Stock must be non-negative"));
            }
        }

        // Validate image_url if provided: valid URL format (if not empty string)
        if let Some(ref url_str) = self.image_url {
            if !url_str.is_empty() {
                // Only validate if URL string is not empty
                if Url::parse(url_str).is_err() {
                    errors.push(FieldError::new("image_url", "Invalid URL format"));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Convert UpdateProductRequest to UpdateProduct
impl From<UpdateProductRequest> for UpdateProduct {
    fn from(req: UpdateProductRequest) -> Self {
        Self {
            name: req.name,
            description: req.description,
            price: req.price,
            stock: req.stock,
            category: req.category,
            image_url: req.image_url,
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

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_product_request_validation_missing_name() {
        let request = CreateProductRequest {
            name: "   ".to_string(), // Empty/whitespace name
            description: None,
            price: Decimal::new(1999, 2),
            stock: 100,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "name");
        assert_eq!(errors[0].message, "Product name is required");
    }

    #[test]
    fn test_create_product_request_validation_name_too_long() {
        let long_name = "a".repeat(256);
        let request = CreateProductRequest {
            name: long_name,
            description: None,
            price: Decimal::new(1999, 2),
            stock: 100,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "name");
        assert!(errors[0].message.contains("255 characters"));
    }

    #[test]
    fn test_create_product_request_validation_description_too_long() {
        let long_desc = "a".repeat(1001);
        let request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: Some(long_desc),
            price: Decimal::new(1999, 2),
            stock: 100,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "description");
        assert!(errors[0].message.contains("1000 characters"));
    }

    #[test]
    fn test_create_product_request_validation_negative_price() {
        let request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: None,
            price: Decimal::new(-100, 0), // -1.00
            stock: 100,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "price");
        assert!(errors[0].message.contains("non-negative"));
    }

    #[test]
    fn test_create_product_request_validation_negative_stock() {
        let request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: None,
            price: Decimal::new(1999, 2),
            stock: -5,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "stock");
        assert!(errors[0].message.contains("non-negative"));
    }

    #[test]
    fn test_create_product_request_validation_invalid_url() {
        let request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: None,
            price: Decimal::new(1999, 2),
            stock: 100,
            category: None,
            image_url: Some("not-a-valid-url".to_string()),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "image_url");
        assert!(errors[0].message.contains("Invalid URL format"));
    }

    #[test]
    fn test_create_product_request_validation_optional_fields_none() {
        let request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: None,
            price: Decimal::ZERO,
            stock: 0,
            category: None,
            image_url: None,
        };

        // Should pass validation with all optional fields as None
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_create_product_request_validation_multiple_errors() {
        let request = CreateProductRequest {
            name: "".to_string(),
            description: Some("a".repeat(1001)),
            price: Decimal::new(-100, 0),
            stock: -5,
            category: None,
            image_url: Some("invalid-url".to_string()),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 5); // All fields have errors
    }

    #[test]
    fn test_create_product_request_to_create_product_conversion() {
        let request = CreateProductRequest {
            name: "Test Product".to_string(),
            description: Some("A test".to_string()),
            price: Decimal::new(1999, 2),
            stock: 50,
            category: Some("Test".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
        };

        let create_product: CreateProduct = request.into();

        assert_eq!(create_product.name, "Test Product");
        assert_eq!(create_product.description, Some("A test".to_string()));
        assert_eq!(create_product.price, Decimal::new(1999, 2));
        assert_eq!(create_product.stock, 50);
        assert_eq!(create_product.category, Some("Test".to_string()));
        assert_eq!(create_product.image_url, Some("https://example.com/image.jpg".to_string()));
    }

    // Tests for UpdateProductRequest

    #[test]
    fn test_update_product_request_validation_all_none() {
        let request = UpdateProductRequest {
            name: None,
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: None,
        };

        // Should pass validation with all fields as None (partial update with no changes)
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_product_request_validation_single_field_name() {
        let request = UpdateProductRequest {
            name: Some("Updated Name".to_string()),
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_product_request_validation_multiple_fields() {
        let request = UpdateProductRequest {
            name: Some("Updated Name".to_string()),
            description: Some("Updated description".to_string()),
            price: Some(Decimal::new(2999, 2)),
            stock: Some(200),
            category: None,
            image_url: Some("https://example.com/new.jpg".to_string()),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_product_request_validation_empty_name() {
        let request = UpdateProductRequest {
            name: Some("   ".to_string()), // Empty/whitespace name
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "name");
        assert_eq!(errors[0].message, "Product name cannot be empty");
    }

    #[test]
    fn test_update_product_request_validation_name_too_long() {
        let long_name = "a".repeat(256);
        let request = UpdateProductRequest {
            name: Some(long_name),
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "name");
        assert!(errors[0].message.contains("255 characters"));
    }

    #[test]
    fn test_update_product_request_validation_description_too_long() {
        let long_desc = "a".repeat(1001);
        let request = UpdateProductRequest {
            name: None,
            description: Some(long_desc),
            price: None,
            stock: None,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "description");
        assert!(errors[0].message.contains("1000 characters"));
    }

    #[test]
    fn test_update_product_request_validation_negative_price() {
        let request = UpdateProductRequest {
            name: None,
            description: None,
            price: Some(Decimal::new(-100, 0)), // -1.00
            stock: None,
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "price");
        assert!(errors[0].message.contains("non-negative"));
    }

    #[test]
    fn test_update_product_request_validation_negative_stock() {
        let request = UpdateProductRequest {
            name: None,
            description: None,
            price: None,
            stock: Some(-5),
            category: None,
            image_url: None,
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "stock");
        assert!(errors[0].message.contains("non-negative"));
    }

    #[test]
    fn test_update_product_request_validation_invalid_url() {
        let request = UpdateProductRequest {
            name: None,
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: Some("not-a-valid-url".to_string()),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "image_url");
        assert!(errors[0].message.contains("Invalid URL format"));
    }

    #[test]
    fn test_update_product_request_validation_empty_url_allowed() {
        let request = UpdateProductRequest {
            name: None,
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: Some("".to_string()), // Empty string should be allowed
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_product_request_validation_none_fields_not_validated() {
        // Test that None fields don't trigger validation
        let request = UpdateProductRequest {
            name: None,            // Not validated
            description: None,     // Not validated
            price: None,           // Not validated
            stock: None,           // Not validated
            category: None,
            image_url: None,       // Not validated
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_update_product_request_validation_multiple_errors() {
        let request = UpdateProductRequest {
            name: Some("".to_string()),
            description: Some("a".repeat(1001)),
            price: Some(Decimal::new(-100, 0)),
            stock: Some(-5),
            category: None,
            image_url: Some("invalid-url".to_string()),
        };

        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 5); // All provided fields have errors
    }

    #[test]
    fn test_update_product_request_to_update_product_conversion() {
        let request = UpdateProductRequest {
            name: Some("Updated Product".to_string()),
            description: Some("An updated description".to_string()),
            price: Some(Decimal::new(2999, 2)),
            stock: Some(75),
            category: Some("Updated Category".to_string()),
            image_url: Some("https://example.com/updated.jpg".to_string()),
        };

        let update_product: UpdateProduct = request.into();

        assert_eq!(update_product.name, Some("Updated Product".to_string()));
        assert_eq!(update_product.description, Some("An updated description".to_string()));
        assert_eq!(update_product.price, Some(Decimal::new(2999, 2)));
        assert_eq!(update_product.stock, Some(75));
        assert_eq!(update_product.category, Some("Updated Category".to_string()));
        assert_eq!(update_product.image_url, Some("https://example.com/updated.jpg".to_string()));
    }

    #[test]
    fn test_update_product_request_partial_conversion() {
        let request = UpdateProductRequest {
            name: Some("Updated Name Only".to_string()),
            description: None,
            price: None,
            stock: None,
            category: None,
            image_url: None,
        };

        let update_product: UpdateProduct = request.into();

        assert_eq!(update_product.name, Some("Updated Name Only".to_string()));
        assert_eq!(update_product.description, None);
        assert_eq!(update_product.price, None);
        assert_eq!(update_product.stock, None);
        assert_eq!(update_product.category, None);
        assert_eq!(update_product.image_url, None);
    }
}
