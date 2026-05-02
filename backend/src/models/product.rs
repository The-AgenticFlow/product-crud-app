//! Product model with SQLx integration
//!
//! This module defines the Product entity and related DTOs

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Product entity representing a product in the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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

/// New product data for insertions
///
/// This struct omits id and timestamps as they are automatically generated
/// by the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub category: Option<String>,
    pub image_url: Option<String>,
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
            created_at: DateTime::default(),
            updated_at: DateTime::default(),
        };

        let json = serde_json::to_string(&product).unwrap();
        assert!(json.contains("Test Product"));
    }

    #[test]
    fn test_new_product_deserialization() {
        let json = r#"{
            "name": "New Product",
            "description": "A new product",
            "price": "29.99",
            "stock": 50,
            "category": "Books",
            "image_url": null
        }"#;

        let new_product: NewProduct = serde_json::from_str(json).unwrap();
        assert_eq!(new_product.name, "New Product");
        assert_eq!(new_product.stock, 50);
    }
}
