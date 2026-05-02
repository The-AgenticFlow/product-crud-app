//! Centralized error handling for the API
//!
//! This module provides custom error types and a standardized error response format

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Field-level validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl FieldError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Main application error type
#[derive(Debug)]
pub enum AppError {
    /// Resource not found (404)
    NotFound {
        resource: String,
        identifier: String,
    },
    /// Bad request (400)
    BadRequest {
        message: String,
    },
    /// Validation errors (400)
    ValidationError {
        errors: Vec<FieldError>,
    },
    /// Internal server error (500)
    InternalError {
        message: String,
        internal_details: String,
    },
    /// Database error (500)
    DatabaseError {
        message: String,
        internal_details: String,
    },
}

impl AppError {
    /// Create a NotFound error for a specific resource
    pub fn not_found(resource: impl Into<String>, identifier: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
            identifier: identifier.into(),
        }
    }

    /// Create a BadRequest error with a message
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
        }
    }

    /// Create a ValidationError with multiple field errors
    pub fn validation(errors: Vec<FieldError>) -> Self {
        Self::ValidationError { errors }
    }

    /// Create a single-field validation error
    pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationError {
            errors: vec![FieldError::new(field, message)],
        }
    }

    /// Create an InternalError that won't leak details to clients
    pub fn internal(message: impl Into<String>, details: impl Into<String>) -> Self {
        Self::InternalError {
            message: message.into(),
            internal_details: details.into(),
        }
    }

    /// Create a DatabaseError that won't leak details to clients
    pub fn database(message: impl Into<String>, details: impl Into<String>) -> Self {
        Self::DatabaseError {
            message: message.into(),
            internal_details: details.into(),
        }
    }

    /// Get HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::BadRequest { .. } | Self::ValidationError { .. } => StatusCode::BAD_REQUEST,
            Self::InternalError { .. } | Self::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get error code string for JSON response
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound { .. } => "NOT_FOUND",
            Self::BadRequest { .. } => "BAD_REQUEST",
            Self::ValidationError { .. } => "VALIDATION_ERROR",
            Self::InternalError { .. } => "INTERNAL_ERROR",
            Self::DatabaseError { .. } => "DATABASE_ERROR",
        }
    }

    /// Get user-facing message (safe to show clients)
    pub fn message(&self) -> String {
        match self {
            Self::NotFound { resource, identifier } => {
                format!("{} with identifier '{}' not found", resource, identifier)
            }
            Self::BadRequest { message } => message.clone(),
            Self::ValidationError { errors } => {
                if errors.len() == 1 {
                    errors[0].message.clone()
                } else {
                    format!("{} validation errors", errors.len())
                }
            }
            Self::InternalError { message, .. } => message.clone(),
            Self::DatabaseError { message, .. } => message.clone(),
        }
    }

    /// Get validation details (only for ValidationError)
    pub fn details(&self) -> Vec<FieldError> {
        match self {
            Self::ValidationError { errors } => errors.clone(),
            _ => vec![],
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound { resource, identifier } => {
                write!(f, "NotFound: {} '{}'", resource, identifier)
            }
            Self::BadRequest { message } => write!(f, "BadRequest: {}", message),
            Self::ValidationError { errors } => {
                write!(f, "ValidationError: {} error(s)", errors.len())
            }
            Self::InternalError { message, internal_details } => {
                write!(f, "InternalError: {} (details: {})", message, internal_details)
            }
            Self::DatabaseError { message, internal_details } => {
                write!(f, "DatabaseError: {} (details: {})", message, internal_details)
            }
        }
    }
}

impl std::error::Error for AppError {}

/// Error response body structure for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<FieldError>,
}

/// Top-level error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        Self {
            error: ErrorBody {
                code: err.error_code().to_string(),
                message: err.message(),
                details: err.details(),
            },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the full error with internal details (server-side only)
        match &self {
            Self::InternalError { internal_details, .. } => {
                tracing::error!("Internal error: {}", internal_details);
            }
            Self::DatabaseError { internal_details, .. } => {
                tracing::error!("Database error: {}", internal_details);
            }
            Self::NotFound { resource, identifier } => {
                tracing::info!("Not found: {} '{}'", resource, identifier);
            }
            Self::BadRequest { message } => {
                tracing::debug!("Bad request: {}", message);
            }
            Self::ValidationError { errors } => {
                tracing::debug!("Validation error: {} error(s)", errors.len());
            }
        }

        let status = self.status_code();
        let body = ErrorResponse::from(self);

        (status, Json(body)).into_response()
    }
}

// Implement From for common error types

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        Self::bad_request(format!("Invalid UUID format: {}", err))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::bad_request(format!("JSON parsing error: {}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_error() {
        let err = AppError::not_found("Product", "abc-123");
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(err.error_code(), "NOT_FOUND");
        assert_eq!(
            err.message(),
            "Product with identifier 'abc-123' not found"
        );
        assert!(err.details().is_empty());
    }

    #[test]
    fn test_bad_request_error() {
        let err = AppError::bad_request("Invalid parameter");
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(err.error_code(), "BAD_REQUEST");
        assert_eq!(err.message(), "Invalid parameter");
    }

    #[test]
    fn test_validation_error_single() {
        let err = AppError::validation_error("email", "Invalid email format");
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(err.error_code(), "VALIDATION_ERROR");
        assert_eq!(err.message(), "Invalid email format");
        assert_eq!(err.details().len(), 1);
    }

    #[test]
    fn test_validation_error_multiple() {
        let err = AppError::validation(vec![
            FieldError::new("email", "Invalid email format"),
            FieldError::new("password", "Password too short"),
        ]);
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(err.error_code(), "VALIDATION_ERROR");
        assert_eq!(err.message(), "2 validation errors");
        assert_eq!(err.details().len(), 2);
    }

    #[test]
    fn test_internal_error() {
        let err = AppError::internal(
            "Internal server error",
            "Database connection failed: timeout",
        );
        assert_eq!(err.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(err.error_code(), "INTERNAL_ERROR");
        assert_eq!(err.message(), "Internal server error");
        // Internal details should NOT be in the public message
        assert!(!err.message().contains("timeout"));
    }

    #[test]
    fn test_database_error() {
        let err = AppError::database("Database error", "Connection pool exhausted");
        assert_eq!(err.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(err.error_code(), "DATABASE_ERROR");
        assert_eq!(err.message(), "Database error");
        // Internal details should NOT be in the public message
        assert!(!err.message().contains("Connection pool"));
    }

    #[test]
    fn test_error_response_serialization() {
        let err = AppError::not_found("Product", "abc-123");
        let response = ErrorResponse::from(err);
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"code\":\"NOT_FOUND\""));
        assert!(json.contains("\"message\":\"Product with identifier 'abc-123' not found\""));
        assert!(json.contains("\"error\""));
    }

    #[test]
    fn test_validation_error_response_serialization() {
        let err = AppError::validation(vec![
            FieldError::new("email", "Invalid email format"),
            FieldError::new("password", "Password too short"),
        ]);
        let response = ErrorResponse::from(err);
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("\"code\":\"VALIDATION_ERROR\""));
        assert!(json.contains("\"details\""));
        assert!(json.contains("\"field\":\"email\""));
        assert!(json.contains("\"field\":\"password\""));
    }

    #[test]
    fn test_display_implementation() {
        let err = AppError::not_found("Product", "abc-123");
        assert_eq!(format!("{}", err), "NotFound: Product 'abc-123'");

        let err = AppError::bad_request("Invalid input");
        assert_eq!(format!("{}", err), "BadRequest: Invalid input");
    }

    #[test]
    fn test_from_uuid_error() {
        let uuid_err = uuid::Uuid::parse_str("invalid").unwrap_err();
        let app_err: AppError = uuid_err.into();
        assert_eq!(app_err.status_code(), StatusCode::BAD_REQUEST);
        assert!(app_err.message().contains("Invalid UUID format"));
    }
}
