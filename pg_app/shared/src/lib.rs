//! Shared types and utilities across the application

#![forbid(unsafe_code)]
#![warn(missing_docs, unreachable_pub)]
/// Module for Data Transfer Objects (DTOs)
pub mod dtos;
/// Module for models
pub mod models;
/// Module for validation utilities and helpers
pub mod validation;

// Re-exports for convenient access
pub use dtos::*;
pub use models::*;
pub use validation::*;

/// Application-wide result type
pub type Result<T> = std::result::Result<T, Error>;

/// Application-wide error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Validation error: {0}")]
    /// Represents a validation error, typically arising from input validation.
        Validation(#[from] validator::ValidationErrors),
    
    /// Represents a database error, typically arising from SQL queries or connection issues.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication error: {0}")]
    /// Represents an authentication error, typically arising from invalid credentials or tokens.
    Auth(String),
    
    #[error("Serialization error: {0}")]
    /// Represents a serialization or deserialization error, typically arising from JSON processing.
    Serde(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    /// Represents a configuration error, typically arising from missing or invalid environment variables.
    Config(String),
    
    #[error("{0}")]
    /// Represents a custom error with a user-defined message.
    Custom(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Custom(value)
    }
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// The URL of the database to connect to.
    pub database_url: String,
    /// The secret key used for signing and verifying JWTs.
    pub jwt_secret: String,
    /// The computational cost of hashing passwords, typically used for security purposes.
    pub password_hash_cost: u32,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| Error::Config("DATABASE_URL must be set".into()))?,
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| Error::Config("JWT_SECRET must be set".into()))?,
            password_hash_cost: std::env::var("PASSWORD_HASH_COST")
                .unwrap_or_else(|_| "12".into())
                .parse()
                .map_err(|_| Error::Config("Invalid PASSWORD_HASH_COST".into()))?,
        })
    }
}