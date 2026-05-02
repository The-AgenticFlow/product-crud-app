//! Database operations and connection pool management
//!
//! This module contains:
//! - Database connection pool setup
//! - Query helpers
//! - Transaction management

#[cfg(feature = "database")]
pub mod pool;

#[cfg(feature = "database")]
pub use pool::{create_pool, run_migrations};
