//! Data models for the application
//!
//! This module contains:
//! - Domain entities (Product, etc.)
//! - DTOs (Data Transfer Objects)
//! - Database models
//! - Query parameters
//! - Pagination structures

#[cfg(feature = "database")]
pub mod pagination;
#[cfg(feature = "database")]
pub mod product;
#[cfg(feature = "database")]
pub mod query_params;

#[cfg(feature = "database")]
#[allow(unused_imports)]
pub use product::{CreateProduct, CreateProductRequest, Product, UpdateProduct};
#[cfg(feature = "database")]
#[allow(unused_imports)]
pub use pagination::{PaginatedResponse, Pagination};
#[cfg(feature = "database")]
#[allow(unused_imports)]
pub use query_params::ListProductsQuery;
