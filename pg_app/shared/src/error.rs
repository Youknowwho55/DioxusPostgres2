//! Central error handling and configuration for the application
//!
//! Defines the application-wide error type and configuration loading.

use thiserror::Error;
use validator::ValidationErrors;

/// Application-wide result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Unified error type for all application errors
#[derive(Error, Debug)]
pub enum Error {
    /// Input validation failed (400 Bad Request)
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
    
    /// Database operation failed (500 Internal Server Error)
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    /// Authentication/authorization failure (401 Unauthorized/403 Forbidden)
    #[error("Authentication error: {0}")]
    Auth(String),
    
    /// Data serialization/deserialization failed (400 Bad Request)
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    
    /// Configuration loading failed (500 Internal Server Error)
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Custom error case for business logic (400 Bad Request)
    #[error("{0}")]
    Custom(String),
    
    /// Resource not found (404 Not Found)
    #[error("Not found: {0}")]
    NotFound(String),
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Custom(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Custom(value.to_string())
    }
}

/// Application configuration settings
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Database connection URL (e.g., "postgres://user:pass@localhost:5432/db")
    pub database_url: String,
    
    /// Secret key for JWT signing/verification
    pub jwt_secret: String,
    
    /// Cost factor for password hashing (recommended 12-15)
    pub password_hash_cost: u32,
}

impl AppConfig {
    /// Loads configuration from environment variables
    ///
    /// # Errors
    /// Returns `Error::Config` if any required environment variables are missing or invalid
    ///
    /// # Example
    /// ```no_run
    /// use shared::error::AppConfig;
    /// 
    /// let config = AppConfig::from_env().expect("Failed to load config");
    /// ```
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| Error::Config("DATABASE_URL must be set".to_string()))?,
            jwt_secret: std::env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET must be set".to_string())?,
            password_hash_cost: std::env::var("PASSWORD_HASH_COST")
                .unwrap_or_else(|_| "12".into())
                .parse()
                .map_err(|_| "Invalid PASSWORD_HASH_COST".to_string())?,
        })
    }
}