//! Product model for database operations

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
}
