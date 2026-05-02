//! Database operations and connection pool management
//!
//! This module contains:
//! - Database connection pool setup
//! - Query helpers
//! - Transaction management
//! - Product repository operations

#[cfg(feature = "database")]
pub mod pool;

#[cfg(feature = "database")]
pub mod products;

#[cfg(feature = "database")]
pub use pool::{create_pool, run_migrations};

#[cfg(feature = "database")]
#[allow(unused_imports)]
pub use products::{count_products, get_product_by_id, list_products, ProductRepositoryError};
