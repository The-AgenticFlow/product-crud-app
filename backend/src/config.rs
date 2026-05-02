//! Application configuration module
//!
//! Handles loading and managing configuration from environment variables

use std::env;

/// Database configuration loaded from environment variables
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[allow(dead_code)]
impl DatabaseConfig {
    /// Load database configuration from environment variables
    ///
    /// # Panics
    ///
    /// Panics if required environment variables are missing or invalid
    pub fn from_env() -> Self {
        let port = env::var("DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()
            .expect("Invalid DB_PORT");

        Self {
            host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port,
            name: env::var("DB_NAME").unwrap_or_else(|_| "product_db".to_string()),
            user: env::var("DB_USER").unwrap_or_else(|_| "product_user".to_string()),
            password: env::var("DB_PASSWORD")
                .unwrap_or_else(|_| "product_password".to_string()),
        }
    }

    /// Build the database connection URL
    ///
    /// Format: postgresql://user:password@host:port/database
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_from_env() {
        // Set test environment variables
        env::set_var("DB_HOST", "localhost");
        env::set_var("DB_PORT", "5432");
        env::set_var("DB_NAME", "test_db");
        env::set_var("DB_USER", "test_user");
        env::set_var("DB_PASSWORD", "test_pass");

        let config = DatabaseConfig::from_env();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.name, "test_db");
        assert_eq!(config.user, "test_user");
        assert_eq!(config.password, "test_pass");

        // Clean up
        env::remove_var("DB_HOST");
        env::remove_var("DB_PORT");
        env::remove_var("DB_NAME");
        env::remove_var("DB_USER");
        env::remove_var("DB_PASSWORD");
    }

    #[test]
    fn test_connection_url() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            name: "product_db".to_string(),
            user: "product_user".to_string(),
            password: "secret".to_string(),
        };

        assert_eq!(
            config.connection_url(),
            "postgres://product_user:secret@localhost:5432/product_db"
        );
    }
}
