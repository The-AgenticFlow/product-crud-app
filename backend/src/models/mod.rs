//! Data models for the application
//!
//! This module contains:
//! - Domain entities (Product, etc.)
//! - DTOs (Data Transfer Objects)
//! - Database models

#[cfg(feature = "database")]
pub mod product;

#[cfg(feature = "database")]
pub use product::{CreateProduct, Product, UpdateProduct};
