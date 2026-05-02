//! Data models for the application
//!
//! This module contains:
//! - Domain entities (Product)
//! - DTOs (Data Transfer Objects)
//! - Database models

pub mod product;

// Re-export commonly used types
#[allow(unused_imports)]
pub use product::{NewProduct, Product};
