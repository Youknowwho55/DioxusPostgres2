//! Data Transfer Objects (DTOs) for API communication
//!
//! This module contains structures used for transferring data between
//! the client and server, with validation and serialization support.

use serde::{Serialize, Deserialize};
use validator::Validate;

/// Data Transfer Object for creating new posts
///
/// # Validation Rules
/// - Title: 1-100 characters
/// - Body: 1-5000 characters
///
/// # Example
/// ```json
/// {
///   "title": "My First Post",
///   "body": "This is the content...",
///   "publish": true
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePostDto {
    /// The title of the post (1-100 characters)
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: String,
    
    /// The main content of the post (1-5000 characters)
    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: String,
    
    /// Whether to publish immediately (default: false)
    #[serde(default)]
    pub publish: bool,
}

/// Data Transfer Object for updating existing posts
///
/// All fields are optional - only included fields will be updated.
///
/// # Example
/// ```json
/// {
///   "title": "Updated Title",
///   "publish": true
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePostDto {
    /// New title (1-100 characters if provided)
    #[validate(length(min = 1, max = 100, message = "Title must be 1-100 characters"))]
    pub title: Option<String>,
    
    /// New content (1-5000 characters if provided)
    #[validate(length(min = 1, max = 5000, message = "Content must be 1-5000 characters"))]
    pub body: Option<String>,
    
    /// New publication status
    pub publish: Option<bool>,
}

/// Post response sent to clients
///
/// Contains formatted dates and author information.
#[derive(Debug, Serialize)]
pub struct PostResponse {
    /// Unique post identifier
    pub id: i32,
    
    /// Post title
    pub title: String,
    
    /// Post content
    pub body: String,
    
    /// Creation date in ISO 8601 format
    pub created_at: String,
    
    /// Last update date in ISO 8601 format (if modified)
    pub updated_at: Option<String>,
    
    /// Publication status
    pub is_published: bool,
    
    /// Brief author information (if available)
    pub author: Option<UserBriefResponse>,
}

/// Minimal user information included in post responses
#[derive(Debug, Serialize)]
pub struct UserBriefResponse {
    /// User ID
    pub id: i32,
    
    /// Username
    pub username: String,
    
    /// Combined first and last name
    pub full_name: String,
}

/// User authentication request
///
/// # Security
/// - Passwords should be hashed before transmission in production
///
/// # Example
/// ```json
/// {
///   "username": "johndoe",
///   "password": "securepassword123"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginDto {
    /// Username (3-50 characters)
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    /// Password (minimum 8 characters)
    #[validate(length(min = 8))]
    pub password: String,
}

/// New user registration request
///
/// # Validation Rules
/// - Username: 3-50 alphanumeric + underscore
/// - Names: 1-100 characters
/// - Email: Valid format
/// - Password: Minimum 8 characters
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserDto {
    /// Unique username (3-50 characters)
    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,
    
    /// First name (1-100 characters)
    #[validate(length(min = 1, max = 100, message = "First name must be 1-100 characters"))]
    pub first_name: String,
    
    /// Last name (1-100 characters)
    #[validate(length(min = 1, max = 100, message = "Last name must be 1-100 characters"))]
    pub last_name: String,
    
    /// Valid email address
    #[validate(email(message = "Must provide a valid email address"))]
    pub email: String,
    
    /// Password (minimum 8 characters)
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

/// Authentication response containing JWT token
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JSON Web Token for authenticated requests
    pub token: String,
    
    /// Authenticated user details
    pub user: UserResponse,
}

/// Complete user information response
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// User ID
    pub id: i32,
    
    /// Username
    pub username: String,
    
    /// First name
    pub first_name: String,
    
    /// Last name
    pub last_name: String,
    
    /// Email address
    pub email: String,
    
    /// Role (admin, loan_officer, processor)
    pub role: String,
    
    /// Account creation date (ISO 8601)
    pub created_at: String,
    
    /// Last login date (ISO 8601) if available
    pub last_login: Option<String>,
}